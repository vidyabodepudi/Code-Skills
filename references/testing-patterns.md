# Testing Patterns — CodeHands Reference

> Framework-specific testing patterns and best practices.

## Universal Patterns

### Arrange-Act-Assert (AAA)
```
// Arrange: Set up test data and conditions
const user = createTestUser({ email: 'test@example.com' });

// Act: Execute the behavior under test
const result = await loginUser(user.email, 'password123');

// Assert: Verify the outcome
expect(result.status).toBe(200);
expect(result.body.token).toBeDefined();
```

### Given-When-Then (BDD)
```
Given a registered user with valid credentials
When they submit the login form
Then they receive an authentication token
And they are redirected to the dashboard
```

### DAMP Test Naming
```
// Pattern: test_[action]_[condition]_[expected_result]
test_login_with_valid_credentials_returns_token()
test_login_with_expired_token_returns_401()
test_register_with_duplicate_email_returns_409()
```

## JavaScript / TypeScript

### Jest
```javascript
describe('UserService', () => {
  describe('register', () => {
    it('creates user with hashed password', async () => {
      const result = await userService.register({
        email: 'test@example.com',
        password: 'SecurePass123!'
      });

      expect(result.id).toBeDefined();
      expect(result.password).not.toBe('SecurePass123!');
      expect(await bcrypt.compare('SecurePass123!', result.password)).toBe(true);
    });

    it('rejects duplicate email with 409', async () => {
      await userService.register({ email: 'taken@example.com', password: 'Pass123!' });

      await expect(
        userService.register({ email: 'taken@example.com', password: 'Pass456!' })
      ).rejects.toThrow(ConflictError);
    });
  });
});
```

### React Testing Library
```javascript
import { render, screen, fireEvent } from '@testing-library/react';

test('submit button disabled when form is invalid', () => {
  render(<LoginForm />);
  const submitButton = screen.getByRole('button', { name: /submit/i });
  expect(submitButton).toBeDisabled();

  fireEvent.change(screen.getByLabelText(/email/i), {
    target: { value: 'valid@email.com' }
  });
  fireEvent.change(screen.getByLabelText(/password/i), {
    target: { value: 'ValidPass123!' }
  });

  expect(submitButton).toBeEnabled();
});
```

## Python

### pytest
```python
import pytest
from app.services import UserService

class TestUserRegistration:
    def test_creates_user_with_hashed_password(self, db_session):
        service = UserService(db_session)
        user = service.register(email="test@example.com", password="SecurePass123!")

        assert user.id is not None
        assert user.password != "SecurePass123!"
        assert bcrypt.checkpw(b"SecurePass123!", user.password.encode())

    def test_rejects_duplicate_email(self, db_session):
        service = UserService(db_session)
        service.register(email="taken@example.com", password="Pass123!")

        with pytest.raises(ConflictError):
            service.register(email="taken@example.com", password="Pass456!")

    @pytest.fixture
    def db_session(self):
        session = create_test_session()
        yield session
        session.rollback()
```

## Go

### Standard Library
```go
func TestUserService_Register(t *testing.T) {
    t.Run("creates user with hashed password", func(t *testing.T) {
        svc := NewUserService(testDB)
        user, err := svc.Register("test@example.com", "SecurePass123!")

        if err != nil {
            t.Fatalf("unexpected error: %v", err)
        }
        if user.ID == "" {
            t.Error("expected user ID to be set")
        }
        if err := bcrypt.CompareHashAndPassword(
            []byte(user.PasswordHash), []byte("SecurePass123!"),
        ); err != nil {
            t.Error("password not properly hashed")
        }
    })

    t.Run("rejects duplicate email", func(t *testing.T) {
        svc := NewUserService(testDB)
        svc.Register("taken@example.com", "Pass123!")

        _, err := svc.Register("taken@example.com", "Pass456!")
        if !errors.Is(err, ErrConflict) {
            t.Errorf("expected ErrConflict, got %v", err)
        }
    })
}
```

## Rust

### Standard Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_creates_user_with_hashed_password() {
        let db = TestDb::new();
        let svc = UserService::new(&db);

        let user = svc.register("test@example.com", "SecurePass123!")
            .expect("registration failed");

        assert!(!user.id.is_empty());
        assert_ne!(user.password_hash, "SecurePass123!");
        assert!(bcrypt::verify("SecurePass123!", &user.password_hash).unwrap());
    }

    #[test]
    fn register_rejects_duplicate_email() {
        let db = TestDb::new();
        let svc = UserService::new(&db);

        svc.register("taken@example.com", "Pass123!").unwrap();
        let result = svc.register("taken@example.com", "Pass456!");

        assert!(matches!(result, Err(AppError::Conflict(_))));
    }
}
```
