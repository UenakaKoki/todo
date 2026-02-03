import { useEffect, useState } from "react";

const API_BASE = "http://localhost:8000"; // ← FastAPI

function App() {
  const [todos, setTodos] = useState([]);

  useEffect(() => {
    fetch(`${API_BASE}/todos`)
      .then((res) => res.json())
      .then((data) => setTodos(data))
      .catch((err) => {
        console.error(err);
        alert("APIに接続できません");
      });
  }, []);

  return (
    <div style={{ padding: "20px" }}>
      <h1>TODO List</h1>
      <ul>
        {todos.map((todo) => (
          <li key={todo.id}>
            {todo.title} {todo.completed ? "✔" : ""}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
