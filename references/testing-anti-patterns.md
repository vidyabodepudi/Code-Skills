# Testing Anti-Patterns — CodeHands Reference

> Common testing mistakes that reduce test suite value. Flag these during review.

## Structural Anti-Patterns

### 1. The Invisible Assertion
```javascript
// BAD: Test runs code but asserts nothing meaningful
test('user registration', async () => {
  const result = await register({ email: 'a@b.com', password: 'pass' });
  expect(result).toBeDefined(); // Passes for ANYTHING truthy
});

// GOOD: Assert specific behavior
test('user registration creates user with hashed password', async () => {
  const result = await register({ email: 'a@b.com', password: 'pass' });
  expect(result.id).toMatch(/^[a-f0-9-]{36}$/);
  expect(await bcrypt.compare('pass', result.passwordHash)).toBe(true);
});
```

### 2. The Implementation Mirror
```javascript
// BAD: Test mirrors implementation, not behavior
test('getUser calls db.findOne', async () => {
  const spy = jest.spyOn(db, 'findOne');
  await getUser('123');
  expect(spy).toHaveBeenCalledWith({ id: '123' });
});

// GOOD: Test behavior — what it returns, not how it gets there
test('getUser returns user data for valid ID', async () => {
  const user = await getUser('123');
  expect(user.email).toBe('test@example.com');
});
```

### 3. The Shared Mutable State
```javascript
// BAD: Tests depend on execution order
let testUser;

test('create user', async () => {
  testUser = await createUser({ email: 'test@example.com' });
  expect(testUser).toBeDefined();
});

test('delete user', async () => {
  await deleteUser(testUser.id); // Fails if previous test fails
});

// GOOD: Each test creates its own state
test('delete user removes from database', async () => {
  const user = await createUser({ email: 'test@example.com' });
  await deleteUser(user.id);
  expect(await getUser(user.id)).toBeNull();
});
```

### 4. The Mock Fortress
```javascript
// BAD: Mocking everything
test('processOrder', () => {
  jest.mock('./db');
  jest.mock('./email');
  jest.mock('./payment');
  jest.mock('./inventory');
  jest.mock('./logger');
  // At this point you're testing that mocks call mocks
});

// GOOD: Mock external boundaries only
test('processOrder charges card and sends confirmation', async () => {
  const mockPayment = createMockPaymentGateway();
  const order = await processOrder(testOrder, { payment: mockPayment });
  expect(mockPayment.charges).toHaveLength(1);
  expect(order.status).toBe('confirmed');
});
```

### 5. The Snapshot Monster
```javascript
// BAD: Snapshot of complex, frequently-changing object
test('user profile', () => {
  const profile = getUserProfile('123');
  expect(profile).toMatchSnapshot(); // 200-line snapshot nobody reads
});

// GOOD: Assert specific fields that matter
test('user profile includes display name and avatar', () => {
  const profile = getUserProfile('123');
  expect(profile.displayName).toBe('John Doe');
  expect(profile.avatarUrl).toMatch(/^https:\/\//);
});
```

### 6. The Flaky Timer
```javascript
// BAD: Depends on real time
test('cache expires after 5 seconds', async () => {
  cache.set('key', 'value', { ttl: 5000 });
  await sleep(5100); // Flaky on slow CI
  expect(cache.get('key')).toBeNull();
});

// GOOD: Control time
test('cache expires after TTL', () => {
  jest.useFakeTimers();
  cache.set('key', 'value', { ttl: 5000 });
  jest.advanceTimersByTime(5001);
  expect(cache.get('key')).toBeNull();
});
```

### 7. The Kitchen Sink Test
```javascript
// BAD: One test covers everything
test('user flow', async () => {
  const user = await register(data);
  expect(user.id).toBeDefined();
  await verify(user.id);
  const token = await login(data);
  expect(token).toBeDefined();
  const profile = await getProfile(token);
  expect(profile.verified).toBe(true);
  await updateProfile(token, newData);
  const updated = await getProfile(token);
  expect(updated.name).toBe(newData.name);
  // If line 3 fails, you lose signal about lines 4-8
});

// GOOD: Separate tests for each behavior
test('registration creates unverified user', ...);
test('verification marks user as verified', ...);
test('login returns token for verified user', ...);
test('profile update changes user data', ...);
```

## Process Anti-Patterns

### 8. Tests After Code ("Test-Last Development")
Writing tests after code means you test the implementation, not the requirement. The test is shaped by the code, not by the behavior.

### 9. Coverage Theater
100% coverage with meaningless assertions is worse than 60% coverage with rigorous assertions. Coverage measures execution, not verification.

### 10. Test Suite Abandonment
A test suite where half the tests are skipped (`xit`, `@skip`, `// TODO`) is a trust-destroying artifact. Fix or delete — never ignore.
