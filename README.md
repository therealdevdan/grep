# Minigrep - мини-утилита для поиска текста в файлах

Простой аналог `grep` на Rust, который ищет текст в файле с возможностью игнорирования регистра.

## Особенности

- Поиск текста с учетом регистра (по умолчанию)
- Поиск без учета регистра (при установке переменной среды `IGNORE_CASE`)
- Простота использования
- Четкие сообщения об ошибках

## Установка

1. Клонируйте репозиторий:
```bash
git clone https://github.com/ваш-репозиторий/minigrep.git
cd minigrep
```

2. Соберите проект:
```bash
cargo build --release
```

## Использование

### Базовый синтаксис:
```bash
cargo run -- [ПОИСКОВАЯ_СТРОКА] [ПУТЬ_К_ФАЙЛУ]
```

### Пример:
```bash
╭─ pwsh   minigrep 
╰─ cargo run -- the poem.txt
Serching for the
In file poem.txt
Then there's a pair of us - don't tell!
To tell your name the livelong day
```

Где:
- `the` - поисковая строка (что ищем)
- `poem.txt` - путь к файлу (где ищем)

### Поиск без учета регистра:
Установите переменную среды `IGNORE_CASE` перед запуском:

#### В Linux/macOS:
```bash
IGNORE_CASE=1 cargo run -- RuSt poem.txt
```

#### В Windows (PowerShell):
```bash
$env:IGNORE_CASE=1; cargo run -- RuSt poem.txt
```

## Пример файла poem.txt
```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

## Обратная связь
Если у вас есть вопросы или предложения по улучшению, пожалуйста, создайте issue в репозитории проекта.

## Лицензия
Этот проект распространяется под лицензией MIT.
