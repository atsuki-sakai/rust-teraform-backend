# Todo API

**Version**: 1.0.0

A simple Todo API built with Rust and Axum

## 目次

- [認証](#認証)
- [エンドポイント](#エンドポイント)
  - [Auth API](#auth-api)
  - [Todos API](#todos-api)
- [スキーマ](#スキーマ)

## 認証

このAPIはJWT Bearer認証を使用します。

認証が必要なエンドポイントでは、以下のヘッダーを含めてください:

```
Authorization: Bearer <access_token>
```

## エンドポイント

### Auth API

#### `POST` /api/v1/auth/login

Login user

**リクエストボディ**

```json
// See LoginRequest schema
```

**レスポンス**

| ステータス | 説明 |
|------------|------|
| 200 | Login successful |
| 401 | Invalid credentials |

---

#### `POST` /api/v1/auth/refresh

Refresh access token

**リクエストボディ**

```json
// See RefreshRequest schema
```

**レスポンス**

| ステータス | 説明 |
|------------|------|
| 200 | Token refreshed successfully |
| 401 | Invalid refresh token |

---

#### `POST` /api/v1/auth/register

Register a new user

**リクエストボディ**

```json
// See RegisterRequest schema
```

**レスポンス**

| ステータス | 説明 |
|------------|------|
| 201 | User registered successfully |
| 400 | Validation error |
| 409 | Email already registered |

---

### Todos API

#### `GET` /api/v1/todos

List all todos for authenticated user

🔒 **認証必須**

**パラメータ**

| 名前 | 位置 | 型 | 必須 | 説明 |
|------|------|-----|------|------|
| page | query | integer |  | Page number (default: 1) |
| per_page | query | integer |  | Items per page (default: 20, max: 100) |

**レスポンス**

| ステータス | 説明 |
|------------|------|
| 200 | List of todos |
| 401 | Unauthorized |

---

#### `POST` /api/v1/todos

Create a new todo

🔒 **認証必須**

**リクエストボディ**

```json
// See CreateTodoRequest schema
```

**レスポンス**

| ステータス | 説明 |
|------------|------|
| 201 | Todo created |
| 400 | Validation error |
| 401 | Unauthorized |

---

#### `GET` /api/v1/todos/{id}

Get a specific todo

🔒 **認証必須**

**パラメータ**

| 名前 | 位置 | 型 | 必須 | 説明 |
|------|------|-----|------|------|
| id | path | string | ✓ | Todo ID |

**レスポンス**

| ステータス | 説明 |
|------------|------|
| 200 | Todo details |
| 401 | Unauthorized |
| 404 | Todo not found |

---

#### `PUT` /api/v1/todos/{id}

Update a todo

🔒 **認証必須**

**パラメータ**

| 名前 | 位置 | 型 | 必須 | 説明 |
|------|------|-----|------|------|
| id | path | string | ✓ | Todo ID |

**リクエストボディ**

```json
// See UpdateTodoRequest schema
```

**レスポンス**

| ステータス | 説明 |
|------------|------|
| 200 | Todo updated |
| 400 | Validation error |
| 401 | Unauthorized |
| 404 | Todo not found |

---

#### `DELETE` /api/v1/todos/{id}

Delete a todo

🔒 **認証必須**

**パラメータ**

| 名前 | 位置 | 型 | 必須 | 説明 |
|------|------|-----|------|------|
| id | path | string | ✓ | Todo ID |

**レスポンス**

| ステータス | 説明 |
|------------|------|
| 204 | Todo deleted |
| 401 | Unauthorized |
| 404 | Todo not found |

---

## スキーマ

### AuthResponse

| フィールド | 型 | 説明 |
|------------|-----|------|
| access_token | string |  |
| expires_in | integer |  |
| refresh_token | string |  |
| token_type | string |  |

### CreateTodoRequest

| フィールド | 型 | 説明 |
|------------|-----|------|
| description | string | null |  |
| title | string |  |

### LoginRequest

| フィールド | 型 | 説明 |
|------------|-----|------|
| email | string |  |
| password | string |  |

### RefreshRequest

| フィールド | 型 | 説明 |
|------------|-----|------|
| refresh_token | string |  |

### RegisterRequest

| フィールド | 型 | 説明 |
|------------|-----|------|
| email | string |  |
| password | string |  |

### Todo

| フィールド | 型 | 説明 |
|------------|-----|------|
| completed | boolean |  |
| created_at | string |  |
| description | string | null |  |
| id | string |  |
| title | string |  |
| updated_at | string |  |
| user_id | string |  |

### TodoListResponse

| フィールド | 型 | 説明 |
|------------|-----|------|
| page | integer |  |
| per_page | integer |  |
| todos | array<TodoResponse> |  |
| total | integer |  |

### TodoResponse

| フィールド | 型 | 説明 |
|------------|-----|------|
| completed | boolean |  |
| created_at | string |  |
| description | string | null |  |
| id | string |  |
| title | string |  |
| updated_at | string |  |

### UpdateTodoRequest

| フィールド | 型 | 説明 |
|------------|-----|------|
| completed | boolean | null |  |
| description | string | null |  |
| title | string | null |  |

### User

| フィールド | 型 | 説明 |
|------------|-----|------|
| created_at | string |  |
| email | string |  |
| id | string |  |
| updated_at | string |  |

### UserResponse

| フィールド | 型 | 説明 |
|------------|-----|------|
| email | string |  |
| id | string |  |

