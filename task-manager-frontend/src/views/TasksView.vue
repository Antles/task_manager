<template>
  <div class="tasks">
    <h2>Tasks</h2>
    <div v-if="loading">Loading tasks...</div>
    <div v-if="error" class="error">{{ error }}</div>
    <template v-else>
      <div class="task-input">
        <input v-model="newTask" @keyup.enter="addTask" placeholder="Add a new task">
        <button @click="addTask">Add Task</button>
      </div>
      <ul v-if="tasks.length">
        <li v-for="task in tasks" :key="task.id" class="task-item">
          <input type="checkbox" :checked="task.completed" @change="updateTask(task)">
          <span :class="{ completed: task.completed }">{{ task.title }}</span>
          <span class="task-status">(Completed: {{ task.completed }})</span>
          <button @click="deleteTask(task.id)" class="delete-btn">Delete</button>
        </li>
      </ul>
      <p v-else>No tasks found. Add a task to get started!</p>
    </template>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'

const tasks = ref([])
const newTask = ref('')
const loading = ref(true)
const error = ref(null)
const router = useRouter()

const API_URL = 'http://localhost:3000'

function getAuthHeader() {
  const token = localStorage.getItem('token')
  if (!token) {
    router.push('/login')
    throw new Error('No token found')
  }
  return { Authorization: `Bearer ${token}` }
}

async function fetchTasks() {
  loading.value = true
  error.value = null
  try {
    const response = await axios.get(`${API_URL}/tasks`, {
      headers: getAuthHeader()
    })
    tasks.value = response.data
  } catch (err) {
    console.error('Error fetching tasks:', err)
    error.value = 'Failed to fetch tasks. Please try again.'
    if (err.response?.status === 401) {
      router.push('/login')
    }
  } finally {
    loading.value = false
  }
}

async function addTask() {
  if (newTask.value.trim()) {
    try {
      const response = await axios.post(`${API_URL}/tasks`,
        { title: newTask.value.trim(), completed: false,},
        { headers: getAuthHeader() }
      )
      tasks.value.push(response.data)
      newTask.value = ''
    } catch (err) {
      console.error('Error adding task:', err)
      error.value = 'Failed to add task. Please try again.'
      if (err.response?.status === 401) {
        router.push('/login')
      }
    }
  }
}

async function updateTask(task) {
  try {
    const response = await axios.patch(
      `${API_URL}/tasks/${task.id}`,
      { completed: !task.completed },
      { headers: getAuthHeader() }
    )
    const updatedTask = response.data
    const index = tasks.value.findIndex(t => t.id === updatedTask.id)
    if (index !== -1) {
      tasks.value[index] = updatedTask
    }
  } catch (err) {
    console.error('Error updating task:', err)
    error.value = 'Failed to update task. Please try again.'
  }
}

async function deleteTask(id) {
  try {
    await axios.delete(`${API_URL}/tasks/${id}`, {
      headers: getAuthHeader()
    })
    tasks.value = tasks.value.filter(task => task.id !== id)
  } catch (err) {
    console.error('Error deleting task:', err)
    error.value = 'Failed to delete task. Please try again.'
  }
}

onMounted(fetchTasks)
</script>


<style scoped>
.tasks {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
}

.task-input {
  display: flex;
  margin-bottom: 20px;
}

.task-input input {
  flex-grow: 1;
  padding: 10px;
  font-size: 16px;
  border: 1px solid #ddd;
  border-radius: 4px 0 0 4px;
}

.task-input button {
  padding: 10px 20px;
  font-size: 16px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 0 4px 4px 0;
  cursor: pointer;
}

.task-input button:hover {
  background-color: #45a049;
}

ul {
  list-style-type: none;
  padding: 0;
}

.task-item {
  display: flex;
  align-items: center;
  padding: 10px;
  border-bottom: 1px solid #eee;
}

.task-item:last-child {
  border-bottom: none;
}

.task-item input[type="checkbox"] {
  margin-right: 10px;
}

.task-item span {
  flex-grow: 1;
  margin-right: 10px;
}

.completed {
  text-decoration: line-through;
  color: #888;
}

.task-status {
  font-size: 0.8em;
  color: #666;
  margin-right: 10px;
}

.delete-btn {
  padding: 5px 10px;
  background-color: #f44336;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.delete-btn:hover {
  background-color: #d32f2f;
}

.error {
  color: #f44336;
  margin-top: 10px;
}
</style>
