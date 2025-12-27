import { useState, useEffect } from 'react';
import './App.css';
import type { User, UserList, CreateUserRequest, CreateUserResponse } from './types/generated';

const API_BASE = 'http://localhost:3000/api';

function App() {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Form state
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [age, setAge] = useState<string>('');

  // Fetch users
  const fetchUsers = async () => {
    try {
      setLoading(true);
      const response = await fetch(`${API_BASE}/users`);
      if (!response.ok) throw new Error('Failed to fetch users');

      const data: UserList = await response.json();
      setUsers(data.users);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  };

  // Create user
  const createUser = async (e: React.FormEvent) => {
    e.preventDefault();

    const request: CreateUserRequest = {
      name,
      email,
      age: age ? parseInt(age) : null,
    };

    try {
      const response = await fetch(`${API_BASE}/users`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request),
      });

      if (!response.ok) throw new Error('Failed to create user');

      const data: CreateUserResponse = await response.json();
      setUsers([...users, data.user]);

      // Reset form
      setName('');
      setEmail('');
      setAge('');
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    }
  };

  // Delete user
  const deleteUser = async (id: number) => {
    try {
      const response = await fetch(`${API_BASE}/users/${id}`, {
        method: 'DELETE',
      });

      if (!response.ok) throw new Error('Failed to delete user');

      setUsers(users.filter(u => u.id !== id));
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    }
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  if (loading) return <div className="loading">Loading...</div>;

  return (
    <div className="app">
      <h1>🦀 GearMesh Example: Axum + React</h1>
      <p className="subtitle">Type-safe Rust ↔ TypeScript communication</p>

      {error && (
        <div className="error">
          <strong>Error:</strong> {error}
        </div>
      )}

      <div className="container">
        {/* Create User Form */}
        <div className="card">
          <h2>Create New User</h2>
          <form onSubmit={createUser}>
            <div className="form-group">
              <label htmlFor="name">Name *</label>
              <input
                id="name"
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                required
                placeholder="Enter name"
              />
            </div>

            <div className="form-group">
              <label htmlFor="email">Email *</label>
              <input
                id="email"
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                placeholder="Enter email"
              />
            </div>

            <div className="form-group">
              <label htmlFor="age">Age (optional)</label>
              <input
                id="age"
                type="number"
                value={age}
                onChange={(e) => setAge(e.target.value)}
                placeholder="Enter age"
              />
            </div>

            <button type="submit" className="btn-primary">
              Create User
            </button>
          </form>
        </div>

        {/* User List */}
        <div className="card">
          <h2>Users ({users.length})</h2>
          <div className="user-list">
            {users.length === 0 ? (
              <p className="empty">No users yet. Create one!</p>
            ) : (
              users.map((user) => (
                <div key={user.id} className="user-item">
                  <div className="user-info">
                    <div className="user-name">{user.name}</div>
                    <div className="user-email">{user.email}</div>
                    {user.age && <div className="user-age">Age: {user.age}</div>}
                  </div>
                  <button
                    onClick={() => deleteUser(user.id)}
                    className="btn-delete"
                    aria-label={`Delete ${user.name}`}
                  >
                    🗑️
                  </button>
                </div>
              ))
            )}
          </div>
        </div>
      </div>

      <footer className="footer">
        <p>
          Types are automatically generated from Rust using{' '}
          <strong>GearMesh</strong>
        </p>
        <p className="tech-stack">
          Backend: Axum • Frontend: React + TypeScript • Type Sharing: GearMesh
        </p>
      </footer>
    </div>
  );
}

export default App;
