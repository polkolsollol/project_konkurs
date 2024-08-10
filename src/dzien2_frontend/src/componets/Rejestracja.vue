<template>
  <div class="max-w-md mx-auto bg-gray-700 p-8 rounded-md shadow-2xl mt-8">
    <h2 class="text-2xl font-bold mb-6 text-center text-white">Zarejestruj się</h2>
    <form @submit.prevent="register" class="space-y-4">
      <div class="mb-4">
        <label class="block text-gray-300 mb-2">Nazwa użytkownika:</label>
        <input
          type="text"
          v-model="username"
          required
          class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500"
        />
      </div>
      <div class="mb-4">
        <label class="block text-gray-300 mb-2">Hasło:</label>
        <input
          type="password"
          v-model="password"
          required
          class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500"
        />
      </div>
      <div class="mb-4">
        <label for="gmail" class="block text-gray-300 mb-2">Gmail:</label>
        <input
          v-model="gmail"
          type="text"
          class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500"
        />
      </div>
      <div class="flex justify-center">
        <button
          type="submit"
          class="w-72 bg-orange-600 text-white px-4 py-2 rounded-md hover:bg-orange-700 focus:outline-none focus:ring-2 focus:ring-orange-500 focus:ring-opacity-50"
        >
          Zarejestruj
        </button>
      </div>
    </form>
  </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';

export default {
  data() {
    return {
      username: "",
      password: "",
      gmail: ""
    };
  },
  methods: {
    async register() {
      await dzien2_backend.dodaj_uzytkownika(this.username, this.password, "user", this.gmail)
        .then((response) => {
          alert(response);
          this.username = "";
          this.password = "";
          this.gmail = "";
        })
        .catch((error) => {
          console.error('Błąd podczas rejestracji:', error);
          alert('Wystąpił błąd podczas rejestracji.');
        });
    }
  }
};
</script>