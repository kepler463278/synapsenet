# SynapseNet v0.1

**Децентрализованная сеть семантической памяти с Proof of Emergence**

SynapseNet — это P2P-сеть, где люди и локальные ИИ обмениваются зёрнами знаний. Интеллект принадлежит обществу. Центра нет. Владельца нет.

## Что это?

- **Grain (зерно)**: Векторное представление + метаданные + подпись + связи
- **Ценность**: Вклад в эмерджентность (Proof of Emergence)
- **Риск-запросы**: Ответы с последствиями и прозрачностью, без инструкций
- **Данные**: Остаются локально; делится только семантика и доказательства
- **Без премайна**: Нет привилегированных ключей. Эмиссия = вклад.

## Быстрый старт

### Установка

**macOS (M2)**
```bash
brew install rustup sqlite cmake pkg-config
rustup default stable
cargo build --release
```

**Linux (Ubuntu)**
```bash
sudo apt-get install build-essential pkg-config libsqlite3-dev cmake
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo build --release
```

**Windows (x64)**
```powershell
# Установите Rust с https://rustup.rs
# Установите Visual Studio Build Tools (C++)
# Установите SQLite DLL и CMake
cargo build --release
```

### Использование

```bash
# Инициализировать локальный узел
./target/release/syn init

# Добавить знания
./target/release/syn add "Rust — это системный язык программирования"
./target/release/syn add путь/к/документу.txt

# Запросить семантическую память
./target/release/syn query "Что такое Rust?"

# Показать пиры
./target/release/syn peers

# Экспортировать снапшот
./target/release/syn export --output out/
```

## Архитектура

```
Grain → Graph → P2P → PoE
  ↓       ↓       ↓      ↓
Vector  Links  Gossip  NGT
```

**Компоненты:**
- **Core**: Grain, Link, Graph, PoE
- **Storage**: SQLite + HNSW индекс
- **P2P**: libp2p (GossipSub)
- **AI**: ONNX эмбеддинги (CPU baseline)
- **Economy**: NGT кредиты
- **Governance**: Policy engine (OK/AnalysisOnly/Curated)

## Proof of Emergence (PoE)

```
NGT(g) = α * N(g) + β * C(g) + γ * log(1 + R(g))
```

Где:
- **N(g)** = Новизна (1 - max_cos_sim с существующими зёрнами)
- **C(g)** = Когерентность (средняя схожесть с релевантными кластерами)
- **R(g)** = Количество переиспользований (как часто зерно попадает в top-k результатов)

Веса по умолчанию: α=0.5, β=0.3, γ=0.2

**Анти-спам**: Если N(g) < τ и C(g) < τ → награда = 0

## Безопасные ответы

**Классы политик:**
- **OK**: Обычный ответ
- **AnalysisOnly**: Только последствия, без пошаговых инструкций для вреда
- **Curated**: Очередь на проверку куратором

## Поддержка платформ

| Компонент | macOS (ARM64) | Linux (x86_64/ARM64) | Windows (x64) |
|-----------|---------------|----------------------|---------------|
| Rust core | ✅ | ✅ | ✅ |
| SQLite | ✅ | ✅ | ✅ |
| HNSW index | ✅ | ✅ | ✅ |
| ONNX CPU | ✅ | ✅ | ✅ |

## Roadmap

- [x] Базовые примитивы (Grain, Link, Graph)
- [x] SQLite хранилище
- [x] HNSW векторный индекс
- [x] CLI (init/add/query)
- [ ] ONNX эмбеддинги
- [ ] P2P сеть (libp2p)
- [ ] Расчёт PoE
- [ ] Policy engine
- [ ] DevNet скрипты
- [ ] E2E тесты
- [ ] GPU ускорение (Metal/CUDA/DirectML)
- [ ] Desktop UI (Tauri)

## Разработка

```bash
# Запустить тесты
cargo test

# Линтер
cargo clippy

# Форматирование
cargo fmt
```

## Лицензия

MIT OR Apache-2.0

## Безопасность

Сообщайте об уязвимостях: security@synapsenet.org

---

*"Интеллект принадлежит обществу. Центра не существует. Владельца не существует."* — GENESIS.txt
