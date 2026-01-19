# Constellation - Technology Stack Documentation

This comprehensive documentation covers the entire technology stack for the Constellation file tagging system built with Tauri 2.x, Rust, and Vue 3.

## Table of Contents

1. [Tauri 2.x Framework](#tauri-2x-framework)
2. [Rust Backend Libraries](#rust-backend-libraries)
3. [Vue 3 Frontend](#vue-3-frontend)
4. [Related Crates & Tools](#related-crates--tools)

---

## Tauri 2.x Framework

### Overview

Tauri enables the creation of small, fast, and secure cross-platform applications using web technologies for the frontend and Rust for the backend. Build smaller, faster, and more secure desktop applications with a web frontend.

**Official Documentation:**
- [Tauri v2 Documentation](https://v2.tauri.app)
- [Tauri Rust API](https://docs.rs/tauri)

### IPC & Commands

#### Defining Tauri Commands

Commands are the primary way to communicate between the frontend and backend in Tauri applications.

**Basic Command Example:**

```rust
#[tauri::command]
fn my_custom_command(value: String) -> String {
    format!("Received: {}", value)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Frontend Invocation (TypeScript):**

```typescript
import { invoke } from '@tauri-apps/api/core'

export async function callMyCommand(value: string): Promise<string> {
    return await invoke('my_custom_command', { value });
}
```

#### Complex Command with State, Async, and Custom Response

```rust
struct Database;

#[derive(serde::Serialize)]
struct CustomResponse {
    message: String,
    other_val: usize,
}

async fn some_other_function() -> Option<String> {
    Some("response".into())
}

#[tauri::command]
async fn my_custom_command(
    window: tauri::Window,
    number: usize,
    database: tauri::State<'_, Database>,
) -> Result<CustomResponse, String> {
    println!("Called from {}", window.label());
    let result: Option<String> = some_other_function().await;
    if let Some(message) = result {
        Ok(CustomResponse {
            message,
            other_val: 42 + number,
        })
    } else {
        Err("No result".into())
    }
}
```

### State Management

Tauri provides built-in state management for sharing data across commands.

#### Basic State Setup

```rust
struct MyState(String);

#[tauri::command]
fn my_custom_command(state: tauri::State<MyState>) {
    assert_eq!(state.0 == "some state value", true);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(MyState("some state value".into()))
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

#### Mutable State with Mutex

**Synchronous Command:**

```rust
use std::sync::Mutex;

struct AppState {
    counter: u32,
}

#[tauri::command]
fn increase_counter(state: State<'_, Mutex<AppState>>) -> u32 {
    let mut state = state.lock().unwrap();
    state.counter += 1;
    state.counter
}
```

**Async Command:**

```rust
#[tauri::command]
async fn increase_counter(state: State<'_, Mutex<AppState>>) -> Result<u32, ()> {
    let mut state = state.lock().await;
    state.counter += 1;
    Ok(state.counter)
}
```

### Window Management

Tauri provides comprehensive window management APIs for creating, positioning, and controlling application windows.

**Key Window Operations:**
- Create new windows
- Show/hide windows
- Set window size and position
- Toggle fullscreen
- Set window title
- Set window decorations

### File System Access

#### File System Plugin

The File System plugin provides APIs for file and directory operations.

**Installation:**

```toml
# Cargo.toml
[dependencies]
tauri-plugin-fs = "2.0"
```

**Resources:**
- [File System Plugin Documentation](https://v2.tauri.app/plugin/file-system/)
- [File Management in Tauri 2.0](https://quentinwach.com/blog/2024/11/26/files-in-tauri-v2.html)

#### Dialog Plugin

Native system dialogs for opening and saving files along with message dialogs.

**Installation:**

```bash
pnpm add @tauri-apps/plugin-dialog
```

```toml
# Cargo.toml
[dependencies]
tauri-plugin-dialog = "2.0"
```

**Picking a Single File (Rust):**

```rust
app.dialog().file().pick_files(|file_paths| {
    // do something with the optional file paths here
    // the file paths value is `None` if the user closed the dialog
});
```

**Picking Folders (Rust):**

```rust
app.dialog().file().pick_folder(|folder_path| {
    // do something with the optional folder path here
    // the folder path is `None` if the user closed the dialog
});
```

**Blocking Operations in Async Commands:**

```rust
// Use blocking_pick_file() for async commands to avoid deadlocks
```

**Frontend Usage (JavaScript/TypeScript):**

```javascript
import { open, save } from '@tauri-apps/plugin-dialog';

// Open file picker
const selected = await open({
    multiple: false,
    filters: [{
        name: 'Images',
        extensions: ['png', 'jpg', 'jpeg']
    }]
});
```

**Resources:**
- [Dialog Plugin Documentation](https://v2.tauri.app/plugin/dialog/)
- [Rust API Documentation](https://docs.rs/tauri-plugin-dialog/)
- [JavaScript API Reference](https://v2.tauri.app/reference/javascript/dialog/)

### Best Practices

1. **Use TypeScript for Type Safety**: Define TypeScript interfaces for command parameters and responses
2. **Handle Errors Properly**: Always use Result<T, E> return types for commands that can fail
3. **Async for I/O Operations**: Use async commands for database queries, file operations, and network requests
4. **State Management**: Use Mutex or RwLock for shared mutable state
5. **Security**: Follow Tauri's security best practices for IPC communication

---

## Rust Backend Libraries

### rusqlite - SQLite Database Access

#### Overview

Rusqlite is an ergonomic wrapper for using SQLite from Rust, providing safe and efficient database access.

**Resources:**
- [Official Documentation](https://docs.rs/rusqlite/)
- [GitHub Repository](https://github.com/rusqlite/rusqlite)
- [Rust Cookbook - SQLite](https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html)

#### Installation

```toml
[dependencies]
rusqlite = { version = "0.32", features = ["bundled"] }
```

#### Basic Usage

**Opening a Connection:**

```rust
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("constellation.db")?;
    Ok(())
}
```

**Creating Tables:**

```rust
conn.execute(
    "CREATE TABLE IF NOT EXISTS tags (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        group_id INTEGER,
        color TEXT,
        FOREIGN KEY (group_id) REFERENCES tag_groups(id)
    )",
    [],
)?;
```

**Inserting Data:**

```rust
conn.execute(
    "INSERT INTO tags (name, group_id, color) VALUES (?1, ?2, ?3)",
    params![tag_name, group_id, color],
)?;
```

**Querying Data:**

```rust
let mut stmt = conn.prepare("SELECT id, name, color FROM tags WHERE group_id = ?1")?;
let tags = stmt.query_map([group_id], |row| {
    Ok(Tag {
        id: row.get(0)?,
        name: row.get(1)?,
        color: row.get(2)?,
    })
})?;

for tag in tags {
    println!("Tag: {:?}", tag?);
}
```

#### Transactions

Transactions allow executing multiple SQL statements as an atomic unit.

**Basic Transaction:**

```rust
use rusqlite::Transaction;

fn update_tags(conn: &Connection) -> Result<()> {
    let tx = conn.transaction()?;

    tx.execute("INSERT INTO tags (name) VALUES (?1)", params!["tag1"])?;
    tx.execute("INSERT INTO tags (name) VALUES (?1)", params!["tag2"])?;

    tx.commit()?;
    Ok(())
}
```

**Resources:**
- [Transaction Documentation](https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html)
- [Prepared Statements Discussion](https://users.rust-lang.org/t/rusqlite-prepared-statements-in-transactions/41373)

#### Prepared Statements

Prepared statements improve performance and security by reusing compiled SQL queries.

**Creating and Using Prepared Statements:**

```rust
let mut stmt = conn.prepare("SELECT * FROM tags WHERE name = ?1")?;
let tag = stmt.query_row(params!["language"], |row| {
    Ok(Tag {
        id: row.get(0)?,
        name: row.get(1)?,
    })
})?;
```

**Cached Statements:**

```rust
// Set the maximum number of cached prepared statements
conn.set_prepared_statement_cache_capacity(100);
```

#### Best Practices

1. **Use Transactions**: Group related operations in transactions for atomicity
2. **Prepared Statements**: Always use prepared statements to prevent SQL injection
3. **Connection Pooling**: Consider using `r2d2` for connection pooling in multi-threaded applications
4. **Indexes**: Create indexes on frequently queried columns for better performance
5. **PRAGMA Settings**: Configure SQLite PRAGMAs for optimal performance:
   ```rust
   conn.execute_batch("
       PRAGMA journal_mode = WAL;
       PRAGMA synchronous = NORMAL;
       PRAGMA cache_size = -64000;
       PRAGMA foreign_keys = ON;
   ")?;
   ```

### Diesel ORM (Alternative to rusqlite)

#### Overview

Diesel is a safe, extensible ORM and Query Builder for Rust that reduces boilerplate for database interactions.

**Resources:**
- [Official Website](https://diesel.rs/)
- [Documentation](https://docs.rs/diesel)
- [Getting Started Guide](https://diesel.rs/guides/getting-started)
- [GitHub Repository](https://github.com/diesel-rs/diesel)

#### Installation

```toml
[dependencies]
diesel = { version = "2.2", features = ["sqlite", "returning_clauses_for_sqlite_3_35"] }
```

#### Features for SQLite

- **sqlite**: Enables the diesel sqlite backend
- **returning_clauses_for_sqlite_3_35**: Enables support for RETURNING clauses (requires SQLite 3.35.0+)

#### Migrations

Migrations allow you to evolve the database schema over time.

```bash
# Install diesel CLI
cargo install diesel_cli --no-default-features --features sqlite

# Setup diesel
diesel setup

# Create a migration
diesel migration generate create_tags

# Run migrations
diesel migration run
```

**Migration Files Structure:**
- `up.sql`: Apply changes
- `down.sql`: Revert changes

**Resources:**
- [Build with Diesel ORM and SQLite](https://developerlife.com/2024/11/28/rust-sqlite-diesel/)

---

### tokio - Async Runtime

#### Overview

Tokio is an event-driven, non-blocking I/O platform for writing asynchronous applications with Rust.

**Resources:**
- [Official Website](https://tokio.rs/)
- [Documentation](https://docs.rs/tokio)
- [Tutorial - Async in Depth](https://tokio.rs/tokio/tutorial/async)
- [Tutorial - Spawning](https://tokio.rs/tokio/tutorial/spawning)
- [Tutorial - Channels](https://tokio.rs/tokio/tutorial/channels)

#### Installation

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

#### Spawning Tasks

The `spawn` function spawns a new asynchronous task, enabling concurrent execution.

**Basic Task Spawning:**

```rust
use tokio;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        println!("Task running");
    });

    // Wait for task to complete
    handle.await.unwrap();
}
```

**Multiple Tasks:**

```rust
let task1 = tokio::spawn(async { /* work 1 */ });
let task2 = tokio::spawn(async { /* work 2 */ });

tokio::try_join!(task1, task2).unwrap();
```

**Resources:**
- [spawn Documentation](https://docs.rs/tokio/latest/tokio/task/fn.spawn.html)

#### Channels

Tokio provides several channel types for communication between tasks.

**MPSC (Multi-Producer, Single-Consumer):**

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("message").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("Received: {}", message);
    }
}
```

**Oneshot (Single-Producer, Single-Consumer):**

```rust
use tokio::sync::oneshot;

let (tx, rx) = oneshot::channel();

tokio::spawn(async move {
    tx.send("result").unwrap();
});

let result = rx.await.unwrap();
```

**Broadcast (Multi-Producer, Multi-Consumer):**

```rust
use tokio::sync::broadcast;

let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    assert_eq!(rx1.recv().await.unwrap(), "message");
});

tx.send("message").unwrap();
```

#### Shared State

**Using Mutex:**

```rust
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(0));

    let data_clone = data.clone();
    tokio::spawn(async move {
        let mut lock = data_clone.lock().await;
        *lock += 1;
    });
}
```

**Resources:**
- [Shared State Tutorial](https://tokio.rs/tokio/tutorial/shared-state)

#### Best Practices

1. **Use `tokio::spawn` for CPU-intensive tasks**: Move blocking operations to separate threads
2. **Channel Selection**: Choose the right channel type for your use case
3. **Avoid Blocking**: Never block in async code; use `tokio::task::spawn_blocking` for blocking operations
4. **Error Handling**: Always handle errors in spawned tasks
5. **Runtime Configuration**: Configure the runtime based on your workload

---

### serde - Serialization Framework

#### Overview

Serde is a framework for serializing and deserializing Rust data structures efficiently and generically.

**Resources:**
- [Official Website](https://serde.rs/)
- [Documentation](https://docs.rs/serde)
- [Using Derive](https://serde.rs/derive.html)
- [Custom Serialization](https://serde.rs/custom-serialization.html)

#### Installation

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

#### Using Derive Macros

**Basic Usage:**

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Tag {
    id: u32,
    name: String,
    color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TagGroup {
    id: u32,
    name: String,
    tags: Vec<Tag>,
}
```

#### JSON Serialization/Deserialization

**Serializing to JSON:**

```rust
use serde_json;

let tag = Tag {
    id: 1,
    name: "Japanese".to_string(),
    color: Some("#FF0000".to_string()),
};

let json = serde_json::to_string(&tag)?;
println!("{}", json);
// Output: {"id":1,"name":"Japanese","color":"#FF0000"}
```

**Deserializing from JSON:**

```rust
let json = r#"{"id":1,"name":"Japanese","color":"#FF0000"}"#;
let tag: Tag = serde_json::from_str(json)?;
```

**Pretty Printing:**

```rust
let json = serde_json::to_string_pretty(&tag)?;
```

#### Field Attributes

**Renaming Fields:**

```rust
#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "userId")]
    user_id: u32,

    #[serde(rename = "firstName")]
    first_name: String,
}
```

**Optional Fields:**

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(default)]
    theme: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    cache_size: Option<usize>,
}
```

#### Custom Serialization

For complex types or specific formatting requirements:

```rust
use serde::{Serializer, Deserializer};

impl Serialize for CustomType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Custom serialization logic
    }
}
```

#### Best Practices

1. **Use Derive Macros**: Leverage `#[derive(Serialize, Deserialize)]` for automatic implementation
2. **Type Safety**: Serde is type-safe and compile-time checked
3. **Error Handling**: Always handle deserialization errors
4. **Performance**: Serde is zero-cost abstraction with no runtime overhead
5. **Format Support**: Works with JSON, YAML, TOML, MessagePack, and more

---

### windows-rs - Windows API Bindings

#### Overview

Windows-rs (Rust for Windows) provides Rust language projections and bindings for Windows APIs.

**Resources:**
- [GitHub Repository](https://github.com/microsoft/windows-rs)
- [API Documentation](https://microsoft.github.io/windows-docs-rs/)

#### Installation

```toml
[dependencies]
windows = { version = "0.63", features = ["Win32_Storage_FileSystem", "Win32_System_Ioctl"] }
```

#### USN Journal API

The USN (Update Sequence Number) Journal is a Windows feature that tracks changes to files on NTFS volumes.

**USN Journal Rust Crates:**
- [usn-journal-rs](https://crates.io/crates/usn-journal-rs) - Safe, ergonomic abstractions for USN journal access
- [usn-parser](https://lib.rs/crates/usn-parser) - Command-line utility for parsing USN Change Journal

**Key Structures:**
- `USN_JOURNAL_DATA` in `windows::Wdk::Storage::FileSystem`
- `USN_JOURNAL_DATA_V2` in `windows::Win32::System::Ioctl`

**usn-journal-rs Features:**

```toml
[dependencies]
usn-journal-rs = "0.2"
```

This crate provides:
- Safe abstractions for accessing USN change journal
- Efficient enumeration of file entries
- Monitoring file system changes on Windows
- Access to MFT (Master File Table) records on NTFS volumes

**Resources:**
- [usn-journal-rs Documentation](https://docs.rs/usn-journal-rs/latest/usn_journal_rs/)
- [USN_JOURNAL_DATA Documentation](https://microsoft.github.io/windows-docs-rs/doc/windows/Wdk/Storage/FileSystem/struct.USN_JOURNAL_DATA.html)

#### Best Practices

1. **Feature Flags**: Only include the Windows APIs you need to reduce compile times
2. **Error Handling**: Windows APIs return Result types; handle errors appropriately
3. **Safety**: Use safe abstractions when available (like usn-journal-rs)
4. **Platform-Specific Code**: Use `#[cfg(target_os = "windows")]` for Windows-only code

---

## Vue 3 Frontend

### Overview

Vue 3 is a progressive JavaScript framework for building user interfaces, designed for incremental adoption and scalability.

**Resources:**
- [Vue 3 Documentation](https://vuejs.org/)
- [Composition API FAQ](https://github.com/vuejs/docs/blob/main/src/guide/extras/composition-api-faq.md)

### Composition API with `<script setup>`

The recommended way to write Vue 3 components using the Composition API.

#### Basic Component Structure

```vue
<script setup>
import { ref, onMounted } from 'vue'

// Reactive state
const count = ref(0)

// Functions that mutate state
function increment() {
    count.value++
}

// Lifecycle hooks
onMounted(() => {
    console.log(`The initial count is ${count.value}.`)
})
</script>

<template>
    <button @click="increment">Count is: {{ count }}</button>
</template>
```

### Reactive State

#### Using `ref()`

For primitive values and single references:

```vue
<script setup>
import { ref } from 'vue'

const message = ref('Hello')
const count = ref(0)
const user = ref(null)

// Access/modify with .value
count.value++
</script>
```

#### Using `reactive()`

For objects and complex data structures:

```vue
<script setup>
import { reactive } from 'vue'

const state = reactive({
    tags: [],
    selectedFolder: null,
    searchQuery: ''
})

// Direct property access
state.tags.push(newTag)
</script>
```

### Computed Properties

For derived state that depends on reactive data:

```vue
<script setup>
import { ref, computed } from 'vue'

const items = ref([
    { id: 1, name: 'Laptop', price: 999, quantity: 1 },
    { id: 2, name: 'Mouse', price: 29, quantity: 2 }
])

// Read-only computed
const totalItems = computed(() => {
    return items.value.reduce((sum, item) => sum + item.quantity, 0)
})

const subtotal = computed(() => {
    return items.value.reduce((sum, item) =>
        sum + (item.price * item.quantity), 0
    )
})

// Writable computed
const discountCode = ref('')
const finalTotal = computed({
    get() {
        return discountCode.value === 'SAVE10'
            ? subtotal.value * 0.9
            : subtotal.value
    },
    set(value) {
        console.log('Setting total:', value)
    }
})
</script>

<template>
    <div>
        <p>Items: {{ totalItems }}</p>
        <p>Subtotal: ${{ subtotal.toFixed(2) }}</p>
        <input v-model="discountCode" placeholder="Discount code">
        <p>Final: ${{ finalTotal.toFixed(2) }}</p>
    </div>
</template>
```

### Watchers

For reacting to data changes and side effects:

```vue
<script setup>
import { ref, watch, watchEffect } from 'vue'

const searchQuery = ref('')
const results = ref([])

// Watch specific ref
watch(searchQuery, async (newQuery) => {
    if (newQuery.length > 2) {
        results.value = await searchTags(newQuery)
    }
})

// Watch multiple sources
watch([folder, sortOrder], ([newFolder, newOrder]) => {
    loadFiles(newFolder, newOrder)
})

// Immediate execution
watchEffect(() => {
    console.log(`Current query: ${searchQuery.value}`)
})
</script>
```

### Component Communication

#### Props (Parent to Child)

```vue
<!-- Parent.vue -->
<template>
    <TagList :tags="tags" :editable="true" />
</template>

<!-- TagList.vue -->
<script setup>
defineProps({
    tags: {
        type: Array,
        required: true
    },
    editable: {
        type: Boolean,
        default: false
    }
})
</script>
```

#### Events (Child to Parent)

```vue
<!-- Child.vue -->
<script setup>
const emit = defineEmits(['update:tag', 'delete'])

function updateTag(tag) {
    emit('update:tag', tag)
}

function deleteTag(id) {
    emit('delete', id)
}
</script>

<!-- Parent.vue -->
<template>
    <TagEditor
        @update:tag="handleTagUpdate"
        @delete="handleDelete"
    />
</template>
```

#### Provide/Inject (Deep Nesting)

```vue
<!-- App.vue (Provider) -->
<script setup>
import { provide, ref } from 'vue'

const currentUser = ref({ name: 'John' })
provide('user', currentUser)
</script>

<!-- NestedComponent.vue (Consumer) -->
<script setup>
import { inject } from 'vue'

const user = inject('user')
</script>
```

### Lifecycle Hooks

```vue
<script setup>
import {
    onBeforeMount,
    onMounted,
    onBeforeUpdate,
    onUpdated,
    onBeforeUnmount,
    onUnmounted
} from 'vue'

onBeforeMount(() => {
    console.log('Before mount')
})

onMounted(() => {
    // Component is mounted and DOM is available
    console.log('Mounted')
})

onBeforeUpdate(() => {
    console.log('Before update')
})

onUpdated(() => {
    console.log('Updated')
})

onBeforeUnmount(() => {
    // Cleanup before component is unmounted
})

onUnmounted(() => {
    console.log('Unmounted')
})
</script>
```

### TypeScript Integration

```vue
<script setup lang="ts">
import { ref, computed } from 'vue'

interface Tag {
    id: number
    name: string
    color?: string
}

const tags = ref<Tag[]>([])
const selectedTag = ref<Tag | null>(null)

const tagCount = computed<number>(() => tags.value.length)

function addTag(tag: Tag): void {
    tags.value.push(tag)
}
</script>
```

### Best Practices

1. **Use `<script setup>`**: More concise and better performance
2. **Composition API**: Preferred over Options API for new projects
3. **Computed vs Methods**: Use computed for derived state, methods for actions
4. **Ref vs Reactive**: Use `ref()` for primitives, `reactive()` for objects
5. **TypeScript**: Add type safety with TypeScript
6. **Composables**: Extract reusable logic into composable functions

---

### Pinia - State Management

#### Overview

Pinia is Vue's official state management library, offering intuitive, type-safe, and flexible stores.

**Resources:**
- [Official Documentation](https://pinia.vuejs.org/)
- [GitHub Repository](https://github.com/vuejs/pinia)

#### Installation

```bash
pnpm add pinia
```

```typescript
// main.ts
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'

const app = createApp(App)
app.use(createPinia())
app.mount('#app')
```

#### Defining Stores - Setup Syntax (Recommended)

The Setup syntax uses Composition API patterns:

```typescript
// stores/tags.ts
import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useTagStore = defineStore('tags', () => {
    // State
    const tags = ref<Tag[]>([])
    const selectedTags = ref<number[]>([])
    const isLoading = ref(false)

    // Getters
    const tagCount = computed(() => tags.value.length)

    const selectedTagObjects = computed(() =>
        tags.value.filter(tag => selectedTags.value.includes(tag.id))
    )

    const getTagsByGroup = computed(() => (groupId: number) =>
        tags.value.filter(tag => tag.groupId === groupId)
    )

    // Actions
    async function loadTags() {
        isLoading.value = true
        try {
            const result = await invoke('get_all_tags')
            tags.value = result
        } catch (error) {
            console.error('Failed to load tags:', error)
        } finally {
            isLoading.value = false
        }
    }

    async function addTag(tag: Omit<Tag, 'id'>) {
        try {
            const newTag = await invoke('create_tag', { tag })
            tags.value.push(newTag)
            return newTag
        } catch (error) {
            console.error('Failed to add tag:', error)
            throw error
        }
    }

    async function deleteTag(tagId: number) {
        try {
            await invoke('delete_tag', { tagId })
            tags.value = tags.value.filter(t => t.id !== tagId)
        } catch (error) {
            console.error('Failed to delete tag:', error)
            throw error
        }
    }

    function toggleTagSelection(tagId: number) {
        const index = selectedTags.value.indexOf(tagId)
        if (index > -1) {
            selectedTags.value.splice(index, 1)
        } else {
            selectedTags.value.push(tagId)
        }
    }

    function clearSelection() {
        selectedTags.value = []
    }

    return {
        // State
        tags,
        selectedTags,
        isLoading,
        // Getters
        tagCount,
        selectedTagObjects,
        getTagsByGroup,
        // Actions
        loadTags,
        addTag,
        deleteTag,
        toggleTagSelection,
        clearSelection
    }
})
```

#### Defining Stores - Options Syntax

The traditional Options API syntax:

```typescript
// stores/counter.ts
import { defineStore } from 'pinia'

export const useCounterStore = defineStore('counter', {
    state: () => ({
        count: 0,
        name: 'Counter Store',
        history: []
    }),

    getters: {
        doubleCount: (state) => state.count * 2,

        doubleCountPlusOne(): number {
            return this.doubleCount + 1
        }
    },

    actions: {
        increment(amount = 1) {
            this.count += amount
            this.history.push({
                action: 'increment',
                value: amount,
                timestamp: Date.now()
            })
        },

        async fetchData() {
            try {
                const response = await fetch('https://api.example.com/data')
                const data = await response.json()
                this.count = data.count
                return data
            } catch (error) {
                console.error('Failed to fetch:', error)
                throw error
            }
        },

        reset() {
            this.$reset()
        }
    }
})
```

#### Using Stores in Components

```vue
<script setup>
import { useTagStore } from '@/stores/tags'
import { storeToRefs } from 'pinia'

const tagStore = useTagStore()

// Destructure state with reactivity
const { tags, selectedTags, isLoading } = storeToRefs(tagStore)

// Destructure actions (no need for storeToRefs)
const { loadTags, addTag, deleteTag } = tagStore

// Or access directly
// tagStore.tags
// tagStore.addTag()

onMounted(() => {
    loadTags()
})

async function handleAddTag() {
    await addTag({
        name: 'New Tag',
        groupId: 1,
        color: '#FF0000'
    })
}
</script>

<template>
    <div>
        <p v-if="isLoading">Loading...</p>
        <div v-else>
            <p>Total tags: {{ tagStore.tagCount }}</p>
            <ul>
                <li v-for="tag in tags" :key="tag.id">
                    {{ tag.name }}
                    <button @click="deleteTag(tag.id)">Delete</button>
                </li>
            </ul>
            <button @click="handleAddTag">Add Tag</button>
        </div>
    </div>
</template>
```

#### State Persistence

For persisting state across sessions, use `pinia-plugin-persistedstate`:

```bash
pnpm add pinia-plugin-persistedstate
```

```typescript
// main.ts
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

app.use(pinia)
```

```typescript
// stores/preferences.ts
export const usePreferencesStore = defineStore('preferences', {
    state: () => ({
        theme: 'light',
        viewMode: 'grid',
        sortOrder: 'name'
    }),
    persist: true  // Enable persistence
})
```

#### Multiple Stores Communication

```typescript
// stores/files.ts
import { useTagStore } from './tags'

export const useFileStore = defineStore('files', () => {
    const files = ref([])
    const tagStore = useTagStore()

    async function getFilesWithTags() {
        const filesData = await invoke('get_files')

        // Use tag store data
        return filesData.map(file => ({
            ...file,
            tagObjects: file.tagIds.map(id =>
                tagStore.tags.find(t => t.id === id)
            )
        }))
    }

    return { files, getFilesWithTags }
})
```

#### Best Practices

1. **Setup Syntax**: Prefer Setup syntax for consistency with Composition API
2. **Store Organization**: One store per domain/feature
3. **TypeScript**: Use TypeScript for type safety
4. **Actions for Mutations**: Always use actions to modify state
5. **Getters for Computed**: Use getters for derived state
6. **Direct State Access**: Can directly mutate state in actions (unlike Vuex)
7. **Modular Stores**: Keep stores focused and composable

---

## Related Crates & Tools

### File System Monitoring

#### notify-rs

Cross-platform filesystem notification library for Rust.

**Resources:**
- [GitHub Repository](https://github.com/notify-rs/notify)
- [Documentation](https://docs.rs/notify/)
- [Crates.io](https://crates.io/crates/notify)

**Installation:**

```toml
[dependencies]
notify = "7.0"
```

**Basic Usage:**

```rust
use notify::{Watcher, RecursiveMode, Result};
use notify::event::{Event, EventKind};

fn watch_directory() -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event>| {
        tx.send(res).unwrap();
    })?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("Event: {:?}", event),
            Err(e) => println!("Error: {:?}", e),
        }
    }

    Ok(())
}
```

**Platform-Specific Implementations:**
- **Windows**: Uses ReadDirectoryChangesW
- **Linux**: Uses inotify
- **macOS**: Uses FSEvents
- **Polling**: Available as fallback on all platforms

**Note for Windows:** While notify-rs is cross-platform, for your Constellation project you may want to use USN Journal directly (via usn-journal-rs) for more accurate and efficient file system monitoring on Windows.

---

### Image Processing & Thumbnails

#### image-rs

Encoding and decoding images in Rust with support for various formats.

**Resources:**
- [GitHub Repository](https://github.com/image-rs/image)
- [Documentation](https://docs.rs/image/)
- [Thumbnail Function](https://docs.rs/image/latest/image/imageops/fn.thumbnail.html)
- [Resize Function](https://docs.rs/image/latest/image/imageops/fn.resize.html)

**Installation:**

```toml
[dependencies]
image = "0.25"
```

**Creating Thumbnails:**

```rust
use image;

fn create_thumbnail(input_path: &str, output_path: &str, max_dimension: u32) -> Result<(), image::ImageError> {
    let img = image::open(input_path)?;
    let thumbnail = img.thumbnail(max_dimension, max_dimension);
    thumbnail.save(output_path)?;
    Ok(())
}
```

**Resizing with Specific Dimensions:**

```rust
use image::{imageops, FilterType};

fn resize_image(input_path: &str, output_path: &str, width: u32, height: u32) -> Result<(), image::ImageError> {
    let img = image::open(input_path)?;
    let resized = imageops::resize(&img, width, height, FilterType::Lanczos3);
    resized.save(output_path)?;
    Ok(())
}
```

**Filter Types:**
- `Nearest`: Fast, low quality
- `Triangle`: Bilinear interpolation
- `CatmullRom`: Bicubic interpolation
- `Gaussian`: Smooth, preserves detail
- `Lanczos3`: High quality, slower

**Supported Formats:**
- PNG
- JPEG
- GIF
- WebP
- TIFF
- BMP
- ICO
- HDR

#### fast_image_resize

High-performance image resizing with SIMD instructions.

**Installation:**

```toml
[dependencies]
fast_image_resize = "5.0"
```

**Use Case:** When you need maximum performance for thumbnail generation, especially when processing many images.

---

### Video Thumbnail Generation

#### FFmpeg Bindings

**ez-ffmpeg** - High-level wrapper for FFmpeg:

```toml
[dependencies]
ez-ffmpeg = "0.2"
```

**Resources:**
- [Quick and Easy Video Thumbnail Generation in Rust](https://dev.to/yeauty/quick-and-easy-video-thumbnail-generation-in-rust-3d1c)
- [Leveraging ffmpeg-next and image-rs](https://medium.com/@akinsella/leveraging-ffmpeg-next-and-image-rs-for-multimedia-processing-in-rust-2097d1137d53)

**Example: Extract Video Frame:**

```rust
// Pseudo-code using ez-ffmpeg
fn extract_video_thumbnail(video_path: &str, output_path: &str, timestamp: f64) -> Result<()> {
    // Extract frame at specific timestamp
    // Use image-rs to resize and save
    Ok(())
}
```

**Performance Tips:**
1. Use `-ss` (seek) flag BEFORE `-i` (input) for faster seeking
2. Extract I-frames (keyframes) for better performance
3. Use `-q:v 2` for high-quality JPEG output
4. Seeking can be 3.8x faster than filtering entire video

**Alternative Libraries:**
- **ffmpeg-next**: Lower-level FFmpeg bindings
- **rust-ffmpeg-frame-grabber**: Frame iterator for videos

**Resources:**
- [Extract Thumbnails from Video with FFmpeg](https://www.mux.com/articles/extract-thumbnails-from-a-video-with-ffmpeg)
- [Faster Thumbnail Generation with FFmpeg Seeking](https://sebi.io/posts/2024-12-21-faster-thumbnail-generation-with-ffmpeg-seeking/)

---

## Database Schema Design for Constellation

### Recommended Tables

```sql
-- Tag Groups
CREATE TABLE tag_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL,
    sort_order INTEGER DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Tags
CREATE TABLE tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    group_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (group_id) REFERENCES tag_groups(id) ON DELETE CASCADE,
    UNIQUE (name, group_id)
);

-- File System Items (folders and files)
CREATE TABLE fs_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL UNIQUE,
    item_type TEXT NOT NULL CHECK(item_type IN ('folder', 'file')),
    usn_number INTEGER,  -- USN Journal tracking
    file_ref_number INTEGER,  -- NTFS File Reference Number
    parent_file_ref_number INTEGER,
    custom_thumbnail_path TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Tag Associations
CREATE TABLE item_tags (
    item_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (item_id, tag_id),
    FOREIGN KEY (item_id) REFERENCES fs_items(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Tag Templates
CREATE TABLE tag_templates (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- Template Tags
CREATE TABLE template_tags (
    template_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (template_id, tag_id),
    FOREIGN KEY (template_id) REFERENCES tag_templates(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

-- Thumbnail Cache
CREATE TABLE thumbnail_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id INTEGER NOT NULL UNIQUE,
    thumbnail_path TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    last_accessed INTEGER NOT NULL,
    FOREIGN KEY (item_id) REFERENCES fs_items(id) ON DELETE CASCADE
);

-- Indexes for Performance
CREATE INDEX idx_tags_group_id ON tags(group_id);
CREATE INDEX idx_fs_items_path ON fs_items(path);
CREATE INDEX idx_fs_items_file_ref ON fs_items(file_ref_number);
CREATE INDEX idx_fs_items_usn ON fs_items(usn_number);
CREATE INDEX idx_item_tags_tag_id ON item_tags(tag_id);
CREATE INDEX idx_thumbnail_cache_last_accessed ON thumbnail_cache(last_accessed);
```

### Query Examples

**Search files with multiple tags (AND logic):**

```sql
SELECT f.*
FROM fs_items f
WHERE f.id IN (
    SELECT item_id
    FROM item_tags
    WHERE tag_id IN (1, 2, 3)
    GROUP BY item_id
    HAVING COUNT(DISTINCT tag_id) = 3
);
```

**Search files with any of the tags (OR logic):**

```sql
SELECT DISTINCT f.*
FROM fs_items f
JOIN item_tags it ON f.id = it.item_id
WHERE it.tag_id IN (1, 2, 3);
```

**Get all tags for an item:**

```sql
SELECT t.*, tg.name as group_name, tg.color as group_color
FROM tags t
JOIN item_tags it ON t.id = it.tag_id
JOIN tag_groups tg ON t.group_id = tg.id
WHERE it.item_id = ?
ORDER BY tg.sort_order, t.name;
```

---

## Project Structure Recommendation

```
Constellation/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/           # Tauri command handlers
│   │   │   ├── mod.rs
│   │   │   ├── tags.rs
│   │   │   ├── files.rs
│   │   │   └── search.rs
│   │   ├── db/                 # Database layer
│   │   │   ├── mod.rs
│   │   │   ├── schema.rs
│   │   │   ├── tags.rs
│   │   │   └── files.rs
│   │   ├── fs_monitor/         # File system monitoring
│   │   │   ├── mod.rs
│   │   │   └── usn_journal.rs
│   │   ├── thumbnail/          # Thumbnail generation
│   │   │   ├── mod.rs
│   │   │   ├── image.rs
│   │   │   └── video.rs
│   │   └── models/             # Data structures
│   │       ├── mod.rs
│   │       ├── tag.rs
│   │       └── file.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                        # Vue frontend
│   ├── main.ts
│   ├── App.vue
│   ├── components/
│   │   ├── FileExplorer/
│   │   │   ├── FolderTree.vue
│   │   │   ├── FileList.vue
│   │   │   └── Thumbnail.vue
│   │   ├── TagManagement/
│   │   │   ├── TagPanel.vue
│   │   │   ├── TagEditor.vue
│   │   │   └── TagFilter.vue
│   │   └── Common/
│   │       ├── SearchBar.vue
│   │       └── ViewModeToggle.vue
│   ├── stores/
│   │   ├── tags.ts
│   │   ├── files.ts
│   │   └── preferences.ts
│   ├── composables/            # Reusable logic
│   │   ├── useTauri.ts
│   │   ├── useFileSystem.ts
│   │   └── useTags.ts
│   └── types/
│       ├── tag.ts
│       └── file.ts
├── package.json
└── tsconfig.json
```

---

## Getting Started Checklist

### Phase 1: MVP Foundation

- [ ] Set up Tauri 2.x project structure
- [ ] Initialize SQLite database with schema
- [ ] Create basic Tauri commands for:
  - [ ] Tag CRUD operations
  - [ ] File system browsing
  - [ ] Tag-file associations
- [ ] Set up Vue 3 with Pinia
- [ ] Create basic UI layout:
  - [ ] Dual-pane layout
  - [ ] File explorer tree
  - [ ] Tag management panel
- [ ] Implement basic file listing
- [ ] Implement basic tag assignment

### Phase 2: Advanced Features

- [ ] Tag group system
- [ ] Tag templates
- [ ] Auto-complete for tag input
- [ ] Quick filter interface
- [ ] Tag statistics
- [ ] View mode toggle (detail/grid)
- [ ] Sort by tags

### Phase 3: Preview & Monitoring

- [ ] Image thumbnail generation
- [ ] Video thumbnail generation
- [ ] Folder thumbnail system
- [ ] Thumbnail caching with LRU
- [ ] USN Journal monitoring implementation
- [ ] Path change detection and update

### Phase 4: Polish & Optimization

- [ ] Database query optimization
- [ ] UI virtual scrolling
- [ ] Performance profiling
- [ ] Error handling
- [ ] Testing
- [ ] User experience improvements

---

## Additional Resources

### Tauri
- [Tauri GitHub](https://github.com/tauri-apps/tauri)
- [Tauri Blog](https://v2.tauri.app/blog/tauri-20/)
- [Tauri Discord](https://discord.gg/tauri)

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Book](https://rust-lang.github.io/async-book/)

### Vue 3
- [Vue 3 Guide](https://vuejs.org/guide/)
- [Vue School](https://vueschool.io/)
- [VueUse](https://vueuse.org/) - Collection of Vue Composition utilities

### Community
- [Tauri Awesome](https://github.com/tauri-apps/awesome-tauri)
- [Rust Community](https://www.rust-lang.org/community)
- [Vue Community](https://vuejs.org/community/)

---

## Version Information

This documentation is based on the following versions (as of 2026):

- **Tauri**: 2.x (stable)
- **Rust**: 1.75+
- **Vue**: 3.5+
- **Pinia**: 2.x
- **rusqlite**: 0.32+
- **tokio**: 1.x
- **serde**: 1.0
- **windows-rs**: 0.63+
- **image**: 0.25+
- **notify**: 7.0+

Always check for the latest versions in your Cargo.toml and package.json files.
