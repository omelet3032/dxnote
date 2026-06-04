아, 그 의도라면 맞는 생각이야.

지금 코드가:

```rust
match current_id {
    Some(id) => {
        if update_data(...).await.is_ok() {
            original_state.set(current_text);
            list_resource.restart();
        }
    }
    None => {
        if let Ok(new_id) = insert_data(...).await {
            id_state.set(Some(new_id));
            original_state.set(current_text);
            list_resource.restart();
        }
    }
}
```

이런 형태였다면

```text
restart()가 두 군데 중복
```

이니까 리팩토링 욕구가 생기는 게 정상.

---

그런 경우 나는 보통 이렇게 바꿔.

```rust
let saved = match current_id {
    Some(id) => {
        update_data(id, &current_text, pool_cloned)
            .await
            .is_ok()
    }

    None => {
        match insert_data(&current_text, pool_cloned).await {
            Ok(new_id) => {
                id_state.set(Some(new_id));
                true
            }
            Err(_) => false,
        }
    }
};

if saved {
    original_state.set(current_text);
    list_resource.restart();
}
```

이렇게.

---

이 방식의 장점은:

```text
저장 성공
↓
후처리
```

를 한 곳에 모을 수 있다는 거야.

즉:

```rust
original_state.set(current_text);
list_resource.restart();
```

를 한 번만 씀.

---

오히려 내가 신경 쓰는 건

```rust
list_resource.restart();
```

를 `spawn` 바깥으로 빼는 거야.

예를 들면:

```rust
spawn(async move {
    ...
});

list_resource.restart();
```

이건 안 좋아.

왜냐면:

```text
저장 완료 전
↓
목록 새로고침
```

될 수 있으니까.

---

순서상 맞는 건:

```text
DB 저장 성공
↓
original_content 갱신
↓
list_resource.restart()
```

이야.

그래서:

### 좋은 리팩토링

```rust
if saved {
    original_state.set(current_text);
    list_resource.restart();
}
```

### 안 좋은 리팩토링

```rust
spawn(...);

list_resource.restart();
```

---

그리고 한 가지 더.

지금 구조를 보면 사실 `restart()`가 필요한 이유는:

```text
사이드바 제목
수정시간(updated_at)
```

을 다시 가져오기 위해서잖아?

만약 나중에 최적화한다면

```rust
list_resource.restart();
```

대신

```rust
note_list[index].title = ...
note_list[index].updated_at = ...
```

처럼 로컬 상태만 갱신할 수도 있어.

그러면 DB UPDATE 후에 다시 SELECT 안 날려도 됨.

하지만 지금 단계에서는:

```text
저장 성공
→ restart()
```

가 가장 단순하고 유지보수도 쉬워. 지금은 이 방식으로 가는 게 맞아 보인다.
