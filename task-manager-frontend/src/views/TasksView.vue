<template>
  <div class="tasks-view">
    <h1>My Tasks</h1>
    <div class="task-input">
      <input
        v-model="newTask"
        @keyup.enter="addTask"
        placeholder="Add a new task"
        type="text"
      >
      <button @click="addTask" class="add-btn">Add Task</button>
    </div>
    <ul class="task-list">
      <li v-for="task in tasks" :key="task.id" class="task-item">
        <div class="task-content">
          <input
            type="checkbox"
            :checked="task.completed"
            @change="updateTask(task)"
          >
          <span :class="{ completed: task.completed }" :title="task.title">{{ task.title }}</span>
        </div>
        <button @click="deleteTask(task.id)" class="delete-btn">Delete</button>
      </li>
    </ul>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'

const tasks = ref([])
const newTask = ref('')

const API_URL = 'http://localhost:3000'

async function fetchTasks() {
  try {
    const response = await axios.get(`${API_URL}/tasks`, {
      headers: { Authorization: `Bearer ${localStorage.getItem('token')}` }
    })
    tasks.value = response.data
  } catch (error) {
    console.error('Error fetching tasks:', error)
  }
}

async function addTask() {
  if (newTask.value.trim()) {
    try {
      const response = await axios.post(`${API_URL}/tasks`,
        { title: newTask.value.trim(), completed: false },
        { headers: { Authorization: `Bearer ${localStorage.getItem('token')}` } }
      )
      tasks.value.push(response.data)
      newTask.value = ''
    } catch (error) {
      console.error('Error adding task:', error)
    }
  }
}

async function updateTask(task) {
  try {
    const response = await axios.patch(
      `${API_URL}/tasks/${task.id}`,
      { completed: !task.completed },
      { headers: { Authorization: `Bearer ${localStorage.getItem('token')}` } }
    )
    const index = tasks.value.findIndex(t => t.id === task.id)
    if (index !== -1) {
      tasks.value[index] = response.data
    }
  } catch (error) {
    console.error('Error updating task:', error)
  }
}

async function deleteTask(id) {
  try {
    await axios.delete(`${API_URL}/tasks/${id}`, {
      headers: { Authorization: `Bearer ${localStorage.getItem('token')}` }
    })
    tasks.value = tasks.value.filter(task => task.id !== id)
  } catch (error) {
    console.error('Error deleting task:', error)
  }
}

onMounted(fetchTasks)
</script>

<style scoped>
.tasks-view {
  width: 100%;
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

h1 {
  font-size: 2rem;
  color: #333;
  margin-bottom: 1.5rem;
}

.task-input {
  display: flex;
  margin-bottom: 1.5rem;
}

input[type="text"] {
  flex-grow: 1;
  padding: 0.5rem;
  font-size: 1rem;
  border: 1px solid #ccc;
  border-radius: 4px 0 0 4px;
}

.add-btn {
  padding: 0.5rem 1rem;
  font-size: 1rem;
  background-color: #39c286;
  color: white;
  border: none;
  border-radius: 0 4px 4px 0;
  cursor: pointer;
  transition: background-color 0.3s;
}

.add-btn:hover {
  background-color: #2eaa76;
}

.task-list {
  list-style-type: none;
  padding: 0;
}

.task-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background-color: #f9f9f9;
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.task-content {
  display: flex;
  align-items: center;
  flex-grow: 1;
  min-width: 0; /* Allow flexbox to shrink below content size */
}

.task-content input[type="checkbox"] {
  margin-right: 1rem;
  flex-shrink: 0;
}

.task-content span {
  font-size: 1rem;
  color: #333;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  margin-right: 1rem;
}

.task-content span.completed {
  text-decoration: line-through;
  color: #888;
}

.delete-btn {
  padding: 0.25rem 0.5rem;
  font-size: 0.875rem;
  background-color: #e74c3c;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.3s;
  flex-shrink: 0;
}

.delete-btn:hover {
  background-color: #c0392b;
}

@media (max-width: 768px) {
  .tasks-view {
    padding: 1rem;
  }

  .task-input {
    flex-direction: column;
  }

  input[type="text"] {
    border-radius: 4px;
    margin-bottom: 0.5rem;
  }

  .add-btn {
    border-radius: 4px;
  }
}
</style>
