<template>
  <div class="container">
    <h1>My Tasks</h1>

    <div class="search-filter">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Search tasks..."
      >
      <select v-model="selectedCategory">
        <option value="All">All Categories</option>
        <option v-for="category in categories" :key="category" :value="category">
          {{ category }}
        </option>
      </select>
    </div>

    <div class="add-task">
      <input
        v-model="newTask.title"
        @keyup.enter="addTask"
        type="text"
        placeholder="Add a new task"
      >
      <select v-model="newTask.category">
        <option v-for="category in categories" :key="category" :value="category">
          {{ category }}
        </option>
      </select>
      <button @click="addTask" class="add-task-btn">Add Task</button>
    </div>

    <ul class="task-list">
      <li v-for="task in filteredTasks" :key="task.id" class="task-item">
        <div class="flex items-center">
          <input
            type="checkbox"
            :checked="task.completed"
            @change="updateTask(task)"
            class="task-checkbox"
          >
          <span class="task-title" :class="{ 'line-through': task.completed }">
            {{ task.title }}
          </span>
          <span class="task-category">{{ task.category }}</span>
        </div>
        <div class="task-actions">
          <button @click="editTask(task)" class="edit-btn">Edit</button>
          <button @click="deleteTask(task.id)" class="delete-btn">Delete</button>
        </div>
      </li>
    </ul>

    <!-- Edit task modal -->
    <div v-if="editingTask" class="modal-overlay">
      <div class="modal-content">
        <h2>Edit Task</h2>
        <input v-model="editingTask.title" type="text" class="edit-input">
        <select v-model="editingTask.category" class="edit-select">
          <option v-for="category in categories" :key="category" :value="category">
            {{ category }}
          </option>
        </select>
        <div class="modal-actions">
          <button @click="saveEditedTask" class="save-btn">Save</button>
          <button @click="cancelEdit" class="cancel-btn">Cancel</button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import axios from 'axios'

const tasks = ref([])
const newTask = ref({ title: '', category: 'Work', completed: false })
const searchQuery = ref('')
const selectedCategory = ref('All')
const categories = ['Work', 'Personal', 'Shopping', 'Health']
const editingTask = ref(null)

const API_URL = 'http://localhost:3000'
const WS_URL = 'ws://localhost:3000/ws'

let ws

const filteredTasks = computed(() => {
  return tasks.value.filter(task => {
    const matchesSearch = task.title.toLowerCase().includes(searchQuery.value.toLowerCase())
    const matchesCategory = selectedCategory.value === 'All' || task.category === selectedCategory.value
    return matchesSearch && matchesCategory
  })
})

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
  if (newTask.value.title.trim()) {
    try {
      await axios.post(`${API_URL}/tasks`,
        {title: newTask.value.title, category: newTask.value.category, completed: false},
        { headers: { Authorization: `Bearer ${localStorage.getItem('token')}` } }
      )

      // Don't add the task here, let the WebSocket handle it
      newTask.value = { title: '', category: 'Work', completed: false }
    } catch (error) {
      console.error('Error adding task:', error)
    }
  }
}

async function updateTask(task) {
  try {
    await axios.patch(
      `${API_URL}/tasks/${task.id}`,
      { completed: !task.completed, category: task.category },
      { headers: { Authorization: `Bearer ${localStorage.getItem('token')}` } }
    )
    // Don't update the task here, let the WebSocket handle it
  } catch (error) {
    console.error('Error updating task:', error)
  }
}

async function deleteTask(id) {
  try {
    await axios.delete(`${API_URL}/tasks/${id}`, {
      headers: { Authorization: `Bearer ${localStorage.getItem('token')}` }
    })
    // Don't remove the task here, let the WebSocket handle it
  } catch (error) {
    console.error('Error deleting task:', error)
  }
}

function editTask(task) {
  editingTask.value = { ...task }
}

async function saveEditedTask() {
  try {
    await axios.patch(
      `${API_URL}/tasks/${editingTask.value.id}`,
      { title: editingTask.value.title, category: editingTask.value.category },
      { headers: { Authorization: `Bearer ${localStorage.getItem('token')}` } }
    )
    // Don't update the task here, let the WebSocket handle it
    editingTask.value = null
  } catch (error) {
    console.error('Error saving edited task:', error)
  }
}

function cancelEdit() {
  editingTask.value = null
}

function setupWebSocket() {
  ws = new WebSocket(WS_URL)

  ws.onmessage = (event) => {
    const data = JSON.parse(event.data)
    if (data.deleted) {
      tasks.value = tasks.value.filter(task => task.id !== data.deleted)
    } else {
      const index = tasks.value.findIndex(t => t.id === data.id)
      if (index !== -1) {
        tasks.value[index] = data
      } else {
        tasks.value.push(data)
      }
    }
  }

  ws.onclose = () => {
    console.log('WebSocket connection closed')
    // Attempt to reconnect after a short delay
    setTimeout(setupWebSocket, 1000)
  }

  ws.onerror = (error) => {
    console.error('WebSocket error:', error)
    ws.close()
  }
}

onMounted(() => {
  fetchTasks()
  setupWebSocket()
})

onUnmounted(() => {
  if (ws) {
    ws.close()
  }
})
</script>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  background-color: #f8fafc;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

h1 {
  @apply text-4xl font-bold mb-8 text-center;
  color: #2c3e50;
}

input[type="text"],
select,
button {
  @apply border rounded-lg p-3 transition-all duration-200 ease-in-out;
  font-size: 16px;
}

input[type="text"] {
  @apply w-full bg-white focus:ring-2 focus:ring-blue-300 focus:border-blue-300;
}

select {
  @apply bg-white cursor-pointer hover:bg-gray-50;
}

button {
  @apply font-semibold text-white bg-blue-500 hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-opacity-50;
}

.search-filter {
  @apply mb-6 flex flex-col sm:flex-row space-y-3 sm:space-y-0 sm:space-x-3;
}

.add-task {
  @apply mb-8 flex flex-col sm:flex-row space-y-3 sm:space-y-0 sm:space-x-3;
}

.task-list {
  @apply space-y-4;
}

.task-item {
  @apply flex items-center justify-between py-4 px-6 bg-white rounded-lg shadow-sm hover:shadow-md transition-shadow duration-200;
}

.task-checkbox {
  @apply mr-3 h-5 w-5 text-blue-500 rounded focus:ring-blue-400;
}

.task-title {
  @apply flex-grow text-gray-800;
}

.task-category {
  @apply text-sm text-gray-500 ml-2;
}

.task-actions {
  @apply flex space-x-2;
}

.task-actions button {
  @apply px-3 py-1 text-sm rounded-full;
}

.edit-btn {
  @apply bg-green-500 hover:bg-green-600;
}

.delete-btn {
  @apply bg-red-500 hover:bg-red-600;
}

.add-task-btn {
  @apply w-full sm:w-auto;
}

.modal-overlay {
  @apply fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center;
}

.modal-content {
  @apply bg-white p-6 rounded-lg shadow-xl w-full max-w-md;
}

.modal-content h2 {
  @apply text-2xl font-bold mb-4;
}

.edit-input,
.edit-select {
  @apply w-full mb-4;
}

.modal-actions {
  @apply flex justify-end space-x-2;
}

.save-btn,
.cancel-btn {
  @apply px-4 py-2;
}

.save-btn {
  @apply bg-green-500 hover:bg-green-600;
}

.cancel-btn {
  @apply bg-gray-300 text-gray-800 hover:bg-gray-400;
}

/* Responsive adjustments */
@media (max-width: 640px) {
  .container {
    @apply px-4;
  }

  .task-item {
    @apply flex-col items-start space-y-2;
  }

  .task-actions {
    @apply self-end;
  }
}
</style>
