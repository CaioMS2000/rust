"""
PDF to Markdown Converter

Arquitetura baseada em contratos (Protocols) com injeção de dependências
para permitir trocar entre diferentes estratégias de conversão:
- LLM puro: manda texto bruto para o modelo processar
- Híbrido: usa regex para estrutura básica, LLM para polimento

Uso:
    python pdf_to_markdown.py --strategy llm --model llama3.1:8b
    python pdf_to_markdown.py --strategy hybrid --model qwen2.5:7b
    python pdf_to_markdown.py --strategy regex  # sem LLM

# Só regex (sem LLM, rápido)
python pdf_to_markdown.py "The Rust Programming Language.pdf" -o saida.md --strategy regex

# LLM puro
python pdf_to_markdown.py "The Rust Programming Language.pdf" -o saida.md --strategy llm --model llama3.1:8b

# Híbrido (regex + LLM)
python pdf_to_markdown.py "The Rust Programming Language.pdf" -o saida.md --strategy hybrid --model qwen2.5:7b

"""

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import Protocol, runtime_checkable, Optional
from pathlib import Path
import re
import json
import subprocess


# =============================================================================
# CONTRATOS (Protocols/Interfaces)
# =============================================================================

@dataclass
class PageContent:
    """Representa o conteúdo extraído de uma página do PDF"""
    page_number: int
    text: str
    tables: list[list[list[str]]] = field(default_factory=list)  # lista de tabelas


@dataclass
class ExtractionResult:
    """Resultado completo da extração do PDF"""
    pages: list[PageContent]
    metadata: dict = field(default_factory=dict)

    @property
    def full_text(self) -> str:
        return "\n\n".join(page.text for page in self.pages)


@dataclass
class ConversionResult:
    """Resultado da conversão para Markdown"""
    markdown: str
    warnings: list[str] = field(default_factory=list)  # problemas encontrados
    confidence: float = 1.0  # 0-1, quão confiante está na conversão


@runtime_checkable
class PDFExtractor(Protocol):
    """Contrato para extratores de PDF"""

    def extract(self, pdf_path: Path, start_page: int = 0, end_page: int | None = None) -> ExtractionResult:
        """Extrai conteúdo do PDF"""
        ...


@runtime_checkable
class MarkdownConverter(Protocol):
    """Contrato para conversores de texto para Markdown"""

    def convert(self, content: ExtractionResult) -> ConversionResult:
        """Converte conteúdo extraído para Markdown"""
        ...

    @property
    def name(self) -> str:
        """Nome da estratégia para logging"""
        ...


@runtime_checkable
class LLMClient(Protocol):
    """Contrato para clientes de LLM"""

    def generate(self, prompt: str, system_prompt: str | None = None) -> str:
        """Gera resposta do LLM"""
        ...


# =============================================================================
# IMPLEMENTAÇÕES - EXTRATOR
# =============================================================================

class PdfPlumberExtractor:
    """Extrator usando pdfplumber - melhor para PDFs com tabelas"""

    def extract(self, pdf_path: Path, start_page: int = 0, end_page: int | None = None) -> ExtractionResult:
        try:
            import pdfplumber
        except ImportError:
            raise ImportError("pdfplumber não instalado. Execute: pip install pdfplumber")

        pages: list[PageContent] = []
        metadata = {}

        with pdfplumber.open(pdf_path) as pdf:
            metadata["total_pages"] = len(pdf.pages)

            actual_end = end_page if end_page else len(pdf.pages)

            for i in range(start_page, min(actual_end, len(pdf.pages))):
                page = pdf.pages[i]

                # Extrai texto
                text = page.extract_text() or ""

                # Extrai tabelas separadamente
                tables = []
                for table in page.extract_tables():
                    if table:  # ignora tabelas vazias
                        tables.append(table)

                pages.append(PageContent(
                    page_number=i + 1,
                    text=text,
                    tables=tables
                ))

        return ExtractionResult(pages=pages, metadata=metadata)


# =============================================================================
# IMPLEMENTAÇÕES - CLIENTES LLM
# =============================================================================

class OllamaClient:
    """Cliente para Ollama (modelos locais)"""

    def __init__(self, model: str = "llama3.1:8b", base_url: str = "http://localhost:11434"):
        self.model = model
        self.base_url = base_url

    def generate(self, prompt: str, system_prompt: str | None = None) -> str:
        """Usa ollama via CLI para evitar dependência de requests"""

        full_prompt = prompt
        if system_prompt:
            full_prompt = f"{system_prompt}\n\n{prompt}"

        try:
            result = subprocess.run(
                ["ollama", "run", self.model, full_prompt],
                capture_output=True,
                text=True,
                timeout=120  # 2 minutos timeout
            )

            if result.returncode != 0:
                raise RuntimeError(f"Ollama error: {result.stderr}")

            return result.stdout.strip()

        except FileNotFoundError:
            raise RuntimeError("Ollama não encontrado. Instale em: https://ollama.ai")
        except subprocess.TimeoutExpired:
            raise RuntimeError("Timeout ao chamar Ollama")


class MockLLMClient:
    """Cliente mock para testes sem LLM real"""

    def generate(self, prompt: str, system_prompt: str | None = None) -> str:
        return f"[MOCK LLM OUTPUT]\n{prompt[:200]}..."


# =============================================================================
# IMPLEMENTAÇÕES - CONVERSORES
# =============================================================================

class RegexOnlyConverter:
    """
    Conversor usando apenas regex/heurísticas.
    Bom para documentos com estrutura previsível (como o Rust Book).
    Não requer LLM.
    """

    @property
    def name(self) -> str:
        return "regex-only"

    def convert(self, content: ExtractionResult) -> ConversionResult:
        warnings: list[str] = []
        markdown_parts: list[str] = []

        for page in content.pages:
            converted = self._convert_page(page, warnings)
            markdown_parts.append(converted)

        markdown = "\n\n---\n\n".join(markdown_parts)

        # Limpeza final
        markdown = self._cleanup(markdown)

        confidence = 1.0 - (len(warnings) * 0.05)  # reduz confiança por warning

        return ConversionResult(
            markdown=markdown,
            warnings=warnings,
            confidence=max(0.3, confidence)
        )

    def _convert_page(self, page: PageContent, warnings: list[str]) -> str:
        text = page.text
        lines = text.split('\n')
        result_lines: list[str] = []
        in_code_block = False
        code_buffer: list[str] = []

        for i, line in enumerate(lines):
            stripped = line.strip()

            # Detecta blocos de código (heurística para Rust)
            if self._looks_like_code_start(stripped, lines, i):
                if not in_code_block:
                    in_code_block = True
                    code_buffer = []
                code_buffer.append(line)
                continue
            elif in_code_block:
                if self._looks_like_code_end(stripped, lines, i):
                    code_buffer.append(line)
                    result_lines.append(f"```rust\n{chr(10).join(code_buffer)}\n```")
                    in_code_block = False
                    code_buffer = []
                else:
                    code_buffer.append(line)
                continue

            # Detecta headers (linhas curtas seguidas de texto)
            if self._is_header(stripped, lines, i):
                level = self._detect_header_level(stripped)
                result_lines.append(f"{'#' * level} {stripped}")
                continue

            # Detecta listas
            if self._is_list_item(stripped):
                result_lines.append(f"- {self._clean_list_item(stripped)}")
                continue

            # Texto normal
            result_lines.append(stripped)

        # Se ainda está em bloco de código, fecha
        if in_code_block and code_buffer:
            result_lines.append(f"```rust\n{chr(10).join(code_buffer)}\n```")
            warnings.append(f"Página {page.page_number}: bloco de código não fechado detectado")

        # Converte tabelas
        for table in page.tables:
            result_lines.append(self._table_to_markdown(table))

        return '\n'.join(result_lines)

    def _looks_like_code_start(self, line: str, lines: list[str], index: int) -> bool:
        """Detecta início de código Rust"""
        code_indicators = [
            r'^fn\s+\w+',           # fn nome
            r'^let\s+(mut\s+)?\w+', # let var
            r'^use\s+\w+',          # use crate
            r'^struct\s+\w+',       # struct Nome
            r'^enum\s+\w+',         # enum Nome
            r'^impl\s+',            # impl
            r'^pub\s+(fn|struct|enum|mod)',  # pub ...
            r'^mod\s+\w+',          # mod nome
            r'^\s*///',             # doc comments
            r'^#\[',                # atributos
        ]
        for pattern in code_indicators:
            if re.match(pattern, line):
                return True
        return False

    def _looks_like_code_end(self, line: str, lines: list[str], index: int) -> bool:
        """Detecta fim de código"""
        # Linha vazia seguida de texto normal
        if not line and index + 1 < len(lines):
            next_line = lines[index + 1].strip()
            if next_line and not self._looks_like_code_start(next_line, lines, index + 1):
                # Verifica se parece texto normal (começa com letra maiúscula, etc)
                if re.match(r'^[A-Z]', next_line) and len(next_line) > 30:
                    return True
        return False

    def _is_header(self, line: str, lines: list[str], index: int) -> bool:
        """Detecta se a linha é um header"""
        if not line:
            return False
        # Headers típicos: curtos, título case ou maiúsculas
        if len(line) < 60 and not line.endswith('.'):
            # Verifica se parece título
            words = line.split()
            if len(words) <= 8:
                # Título case ou maiúsculas
                if line.istitle() or line.isupper():
                    return True
                # Padrões específicos do Rust Book
                if re.match(r'^(Chapter \d+|Section \d+|\d+\.\d+)', line):
                    return True
        return False

    def _detect_header_level(self, line: str) -> int:
        """Detecta nível do header (1-4)"""
        if re.match(r'^Chapter \d+', line):
            return 1
        if re.match(r'^\d+\.\d+\.\d+', line):
            return 3
        if re.match(r'^\d+\.\d+', line):
            return 2
        if line.isupper():
            return 2
        return 3

    def _is_list_item(self, line: str) -> bool:
        """Detecta itens de lista"""
        return bool(re.match(r'^[\-\*\•]\s+|^\d+\.\s+', line))

    def _clean_list_item(self, line: str) -> str:
        """Remove marcador de lista"""
        return re.sub(r'^[\-\*\•]\s+|^\d+\.\s+', '', line)

    def _table_to_markdown(self, table: list[list[str]]) -> str:
        """Converte tabela para formato Markdown"""
        if not table or not table[0]:
            return ""

        # Limpa células
        clean_table = [[cell.strip() if cell else "" for cell in row] for row in table]

        # Calcula larguras
        col_count = max(len(row) for row in clean_table)

        # Normaliza número de colunas
        for row in clean_table:
            while len(row) < col_count:
                row.append("")

        lines = []

        # Header
        header = clean_table[0]
        lines.append("| " + " | ".join(header) + " |")
        lines.append("|" + "|".join(["---"] * col_count) + "|")

        # Rows
        for row in clean_table[1:]:
            lines.append("| " + " | ".join(row) + " |")

        return "\n".join(lines)

    def _cleanup(self, markdown: str) -> str:
        """Limpeza final do markdown"""
        # Remove múltiplas linhas vazias
        markdown = re.sub(r'\n{3,}', '\n\n', markdown)
        # Remove espaços trailing
        markdown = '\n'.join(line.rstrip() for line in markdown.split('\n'))
        return markdown.strip()


class LLMOnlyConverter:
    """
    Conversor usando apenas LLM.
    Manda o texto bruto e deixa o modelo estruturar.
    Simples mas depende muito da capacidade do modelo.
    """

    def __init__(self, llm_client: LLMClient, chunk_size: int = 2000):
        self.llm = llm_client
        self.chunk_size = chunk_size

    @property
    def name(self) -> str:
        return "llm-only"

    def convert(self, content: ExtractionResult) -> ConversionResult:
        warnings: list[str] = []
        markdown_parts: list[str] = []
        total_pages = len(content.pages)

        system_prompt = """Você é um conversor de texto para Markdown.
Converta o texto fornecido para Markdown bem formatado.
Regras:
- Use headers (#, ##, ###) para títulos e seções
- Use ``` para blocos de código (especifique a linguagem, ex: ```rust)
- Use - para listas
- Use | para tabelas
- Preserve o conteúdo original, apenas formate
- Se algo não estiver claro, mantenha como texto simples
- NÃO adicione conteúdo que não existe no original
- NÃO remova conteúdo"""

        for page in content.pages:
            # Divide em chunks se necessário
            text = page.text
            if len(text) > self.chunk_size:
                chunks = self._split_into_chunks(text)
                for i, chunk in enumerate(chunks):
                    print(f"  [LLM] Página {page.page_number}/{total_pages} - chunk {i+1}/{len(chunks)}", end='\r')
                    prompt = f"Converta para Markdown:\n\n{chunk}"
                    try:
                        result = self.llm.generate(prompt, system_prompt)
                        markdown_parts.append(result)
                    except Exception as e:
                        warnings.append(f"Página {page.page_number}, chunk {i}: erro LLM - {e}")
                        markdown_parts.append(chunk)  # fallback: texto original
                print()  # nova linha após terminar os chunks da página
            else:
                print(f"  [LLM] Página {page.page_number}/{total_pages}", end='\r')
                prompt = f"Converta para Markdown:\n\n{text}"
                try:
                    result = self.llm.generate(prompt, system_prompt)
                    markdown_parts.append(result)
                except Exception as e:
                    warnings.append(f"Página {page.page_number}: erro LLM - {e}")
                    markdown_parts.append(text)

        print()  # nova linha final

        markdown = "\n\n".join(markdown_parts)
        confidence = 0.7 if not warnings else 0.5

        return ConversionResult(
            markdown=markdown,
            warnings=warnings,
            confidence=confidence
        )

    def _split_into_chunks(self, text: str) -> list[str]:
        """Divide texto em chunks respeitando parágrafos"""
        paragraphs = text.split('\n\n')
        chunks: list[str] = []
        current_chunk = ""

        for para in paragraphs:
            if len(current_chunk) + len(para) > self.chunk_size:
                if current_chunk:
                    chunks.append(current_chunk.strip())
                current_chunk = para
            else:
                current_chunk += "\n\n" + para

        if current_chunk:
            chunks.append(current_chunk.strip())

        return chunks


class HybridConverter:
    """
    Conversor híbrido: regex para estrutura básica, LLM para polimento.
    Mais robusto que LLM puro, mais flexível que regex puro.
    """

    def __init__(self, llm_client: LLMClient):
        self.llm = llm_client
        self.regex_converter = RegexOnlyConverter()

    @property
    def name(self) -> str:
        return "hybrid"

    def convert(self, content: ExtractionResult) -> ConversionResult:
        warnings: list[str] = []

        # Passo 1: Conversão inicial com regex
        regex_result = self.regex_converter.convert(content)
        warnings.extend(regex_result.warnings)

        # Passo 2: Identifica seções problemáticas
        problematic_sections = self._find_problematic_sections(regex_result.markdown)

        if not problematic_sections:
            # Regex deu conta, não precisa de LLM
            return ConversionResult(
                markdown=regex_result.markdown,
                warnings=warnings,
                confidence=regex_result.confidence
            )

        # Passo 3: Usa LLM apenas nas seções problemáticas
        markdown = regex_result.markdown

        for section in problematic_sections:
            try:
                fixed = self._fix_section_with_llm(section)
                markdown = markdown.replace(section, fixed)
            except Exception as e:
                warnings.append(f"Erro ao processar seção com LLM: {e}")

        # Passo 4: Validação final com LLM (opcional, leve)
        markdown = self._final_polish(markdown, warnings)

        confidence = 0.85 if not warnings else 0.7

        return ConversionResult(
            markdown=markdown,
            warnings=warnings,
            confidence=confidence
        )

    def _find_problematic_sections(self, markdown: str) -> list[str]:
        """Identifica seções que podem precisar de revisão do LLM"""
        problems = []

        # Blocos de código não fechados
        code_blocks = re.findall(r'```[\s\S]*?(?:```|$)', markdown)
        for block in code_blocks:
            if block.count('```') % 2 != 0:
                problems.append(block)

        # Tabelas malformadas
        lines = markdown.split('\n')
        in_table = False
        table_lines = []

        for line in lines:
            if '|' in line:
                in_table = True
                table_lines.append(line)
            elif in_table:
                # Verifica se tabela está bem formada
                if table_lines:
                    col_counts = [l.count('|') for l in table_lines]
                    if len(set(col_counts)) > 1:  # colunas inconsistentes
                        problems.append('\n'.join(table_lines))
                in_table = False
                table_lines = []

        # Linhas muito longas (provavelmente parágrafos não separados)
        for line in lines:
            if len(line) > 500 and '```' not in line:
                problems.append(line)

        return problems[:5]  # limita a 5 para não sobrecarregar

    def _fix_section_with_llm(self, section: str) -> str:
        """Usa LLM para corrigir uma seção específica"""
        prompt = f"""Corrija a formatação Markdown desta seção.
Problemas comuns: blocos de código não fechados, tabelas desalinhadas, parágrafos juntos.

Seção:
{section}

Responda APENAS com a seção corrigida, sem explicações."""

        return self.llm.generate(prompt)

    def _final_polish(self, markdown: str, warnings: list[str]) -> str:
        """Polimento final leve (opcional)"""
        # Por enquanto, apenas limpeza básica
        # Pode adicionar chamada LLM para revisão geral se quiser
        markdown = re.sub(r'\n{3,}', '\n\n', markdown)
        return markdown.strip()


# =============================================================================
# FACTORY E ORQUESTRADOR
# =============================================================================

class PDFToMarkdownPipeline:
    """
    Pipeline principal que orquestra extração e conversão.
    Usa injeção de dependência para permitir trocar estratégias.
    """

    def __init__(
        self,
        extractor: PDFExtractor,
        converter: MarkdownConverter
    ):
        self.extractor = extractor
        self.converter = converter

    def process(
        self,
        pdf_path: Path | str,
        output_path: Path | str | None = None,
        start_page: int = 0,
        end_page: int | None = None
    ) -> ConversionResult:
        """Processa PDF e gera Markdown"""

        pdf_path = Path(pdf_path)

        if not pdf_path.exists():
            raise FileNotFoundError(f"PDF não encontrado: {pdf_path}")

        print(f"[{self.converter.name}] Extraindo PDF: {pdf_path.name}")
        content = self.extractor.extract(pdf_path, start_page, end_page)
        print(f"  → {len(content.pages)} páginas extraídas")

        print(f"[{self.converter.name}] Convertendo para Markdown...")
        result = self.converter.convert(content)
        print(f"  → Confiança: {result.confidence:.0%}")

        if result.warnings:
            print(f"  → Avisos: {len(result.warnings)}")
            for w in result.warnings[:3]:
                print(f"    - {w}")

        if output_path:
            output_path = Path(output_path)
            output_path.write_text(result.markdown, encoding='utf-8')
            print(f"  → Salvo em: {output_path}")

        return result


def create_pipeline(
    strategy: str = "hybrid",
    model: str = "llama3.1:8b"
) -> PDFToMarkdownPipeline:
    """
    Factory para criar pipeline com a estratégia desejada.

    Args:
        strategy: "regex", "llm", ou "hybrid"
        model: modelo Ollama para estratégias que usam LLM
    """

    extractor = PdfPlumberExtractor()

    if strategy == "regex":
        converter = RegexOnlyConverter()
    elif strategy == "llm":
        llm = OllamaClient(model=model)
        converter = LLMOnlyConverter(llm)
    elif strategy == "hybrid":
        llm = OllamaClient(model=model)
        converter = HybridConverter(llm)
    elif strategy == "mock":
        llm = MockLLMClient()
        converter = LLMOnlyConverter(llm)
    else:
        raise ValueError(f"Estratégia desconhecida: {strategy}")

    return PDFToMarkdownPipeline(extractor, converter)


# =============================================================================
# CLI
# =============================================================================

def main():
    import argparse

    parser = argparse.ArgumentParser(
        description="Converte PDF para Markdown",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Exemplos:
  python pdf_to_markdown.py livro.pdf -o saida.md --strategy regex
  python pdf_to_markdown.py livro.pdf -o saida.md --strategy llm --model qwen2.5:7b
  python pdf_to_markdown.py livro.pdf -o saida.md --strategy hybrid --model llama3.1:8b
  python pdf_to_markdown.py livro.pdf --start 10 --end 20  # páginas específicas
        """
    )

    parser.add_argument("pdf", help="Caminho do arquivo PDF")
    parser.add_argument("-o", "--output", help="Arquivo de saída (default: <pdf>.md)")
    parser.add_argument(
        "-s", "--strategy",
        choices=["regex", "llm", "hybrid", "mock"],
        default="hybrid",
        help="Estratégia de conversão (default: hybrid)"
    )
    parser.add_argument(
        "-m", "--model",
        default="llama3.1:8b",
        help="Modelo Ollama para estratégias com LLM (default: llama3.1:8b)"
    )
    parser.add_argument("--start", type=int, default=0, help="Página inicial (0-indexed)")
    parser.add_argument("--end", type=int, help="Página final")
    parser.add_argument("-v", "--verbose", action="store_true", help="Modo verboso")

    args = parser.parse_args()

    # Define output
    pdf_path = Path(args.pdf)
    output_path = args.output or pdf_path.with_suffix('.md')

    # Cria e executa pipeline
    try:
        pipeline = create_pipeline(strategy=args.strategy, model=args.model)
        result = pipeline.process(
            pdf_path=pdf_path,
            output_path=output_path,
            start_page=args.start,
            end_page=args.end
        )

        if args.verbose:
            print(f"\n--- Primeiros 500 caracteres ---")
            print(result.markdown[:500])

    except ImportError as e:
        print(f"Erro de dependência: {e}")
        print("Instale com: pip install pdfplumber")
        exit(1)
    except Exception as e:
        print(f"Erro: {e}")
        exit(1)


if __name__ == "__main__":
    main()
