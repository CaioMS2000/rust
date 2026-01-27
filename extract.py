from pypdf import PdfReader
import sys
import re

file_name = 'The Rust Programming Language.pdf' # está na raiz do projeto
output_file='output.txt'
begin_str='Enums and Pattern Matching'
end_str="In order to provide a well-organized API to your users that is straightforward to use and only exposes exactly what your users will need, let’s now turn to Rust’s modules."

def normalize_text(text):
    """
    Normaliza texto para comparação:
    - Converte para minúsculas
    - Remove quebras de linha extras
    - Normaliza espaços múltiplos para um único espaço
    - Normaliza aspas tipográficas para ASCII
    """
    # Converte para minúsculas
    text = text.lower()
    # Substitui quebras de linha por espaços
    text = text.replace('\n', ' ')
    # Normaliza múltiplos espaços para um único
    text = re.sub(r'\s+', ' ', text)
    # Normaliza diferentes tipos de aspas para ASCII usando códigos Unicode explícitos
    text = text.replace('\u2019', "'")  # ' (aspas tipográficas) -> ' (ASCII)
    text = text.replace('\u2018', "'")  # ' (aspas tipográficas) -> ' (ASCII)
    text = text.replace('\u201c', '"')  # " (aspas tipográficas) -> " (ASCII)
    text = text.replace('\u201d', '"')  # " (aspas tipográficas) -> " (ASCII)
    return text.strip()

def find_in_normalized_text(full_text, search_str, debug=False):
    """
    Procura uma string no texto, normalizando ambos para comparação.
    Retorna o índice no texto original (não normalizado).
    """
    # Normaliza a string de busca
    normalized_search = normalize_text(search_str)

    # Cria um padrão regex flexível baseado nas palavras da busca
    words = normalized_search.split()
    if not words:
        return -1

    # Cria padrão que permite espaços/quebras entre palavras
    # e normaliza aspas tipográficas
    pattern_parts = []
    for word in words:
        # Escapa caracteres especiais do regex
        escaped = re.escape(word)
        # Substitui aspas para aceitar variações (ASCII e tipográficas)
        # Usa o caractere Unicode real U+2019 em vez de escape string
        escaped = escaped.replace("'", "['\u2019]")
        pattern_parts.append(escaped)

    # Junta com padrão flexível de espaços (permite \n, múltiplos espaços, etc)
    pattern = r'\s+'.join(pattern_parts)

    if debug:
        print(f"\n[DEBUG] Padrão regex: {pattern[:100]}")

    # Busca no texto original (case insensitive)
    match = re.search(pattern, full_text, re.IGNORECASE)

    if debug:
        if match:
            print(f"[DEBUG] Encontrado na posição: {match.start()}")
            print(f"[DEBUG] Contexto: {repr(full_text[match.start():match.start()+100])}")
        else:
            print(f"[DEBUG] NÃO encontrado")

    if match:
        return match.start()

    return -1

def extract_text_between_markers(pdf_path, start_marker, end_marker):
    """
    Extrai texto entre duas strings marcadoras de um arquivo PDF.
    Processa o PDF página por página para lidar com arquivos grandes.
    Normaliza espaços e quebras de linha para busca flexível.
    """
    try:
        with open(pdf_path, 'rb') as pdf_file:
            pdf_reader = PdfReader(pdf_file)
            total_pages = len(pdf_reader.pages)

            full_text = ""
            start_found = False
            start_index = -1

            # Processa página por página
            for page_num in range(total_pages):
                page = pdf_reader.pages[page_num]
                page_text = page.extract_text()
                full_text += page_text

                # Verifica se encontrou a string de início (busca normalizada)
                if not start_found:
                    # Ativa debug apenas na primeira tentativa de cada página
                    debug_mode = (page_num <= 70)
                    start_index = find_in_normalized_text(full_text, start_marker, debug=debug_mode)
                    if start_index != -1:
                        start_found = True
                        print(f"String de início encontrada na página {page_num}")

                # Se já encontrou o início, verifica se encontrou o fim
                if start_found:
                    end_index = find_in_normalized_text(full_text[start_index:], end_marker)
                    if end_index != -1:
                        end_index += start_index  # Ajusta para o índice global
                        # Procura o final da frase (até o ponto de interrogação)
                        question_mark = full_text.find('?', end_index)
                        if question_mark != -1 and question_mark - end_index < 50:
                            end_index = question_mark + 1
                        else:
                            end_index += len(normalize_text(end_marker))

                        print(f"String de fim encontrada na página {page_num}")
                        extracted_text = full_text[start_index:end_index]
                        return extracted_text

            # Se chegou aqui, não encontrou uma das strings
            if not start_found:
                raise ValueError(f"Erro: String de início não encontrada no PDF.\nBuscando: '{start_marker}'")
            else:
                raise ValueError(f"Erro: String de fim não encontrada no PDF.\nBuscando: '{end_marker}'")

    except FileNotFoundError:
        print(f"Erro: Arquivo '{pdf_path}' não encontrado.")
        sys.exit(1)
    except Exception as e:
        print(f"Erro ao processar o PDF: {e}")
        sys.exit(1)

def main():
    # Valida se as strings foram definidas
    if not begin_str or not end_str:
        print("Erro: As variáveis begin_str e end_str devem ser definidas.")
        sys.exit(1)

    print(f"Processando arquivo: {file_name}")
    print(f"Buscando texto entre '{begin_str}' e '{end_str}'...")

    try:
        # Extrai o texto
        extracted_content = extract_text_between_markers(file_name, begin_str, end_str)

        # Escreve no arquivo de saída
        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(extracted_content)

        print(f"Sucesso! Texto extraído e salvo em '{output_file}'")
        print(f"Total de caracteres extraídos: {len(extracted_content)}")

    except ValueError as e:
        print(str(e))
        sys.exit(1)

if __name__ == "__main__":
    main()

"""
AI prompt:
leia o arquivo 'output.txt', la eu coloquei o conteúdo original de um pedaço do livro oficial do Rust, eu quero que vc siga o padrão de anotação que eu venho usando no meu arquivo atual e anotar esse conteudo que eu separei.
"""