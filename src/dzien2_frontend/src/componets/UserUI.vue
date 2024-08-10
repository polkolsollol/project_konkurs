<template>
  <div class="p-4">
    <div class="relative z-20 flex justify-start rounded-b-lg">
      <div class="bg-orange-600 p-1 shadow-lgA rounded-b-lg flex space-x-2 items-center hover:bg-orange-700">
        <button class="bg-transparent text-white px-4 py-2 focus:outline-none" @click="show_report">Wyświetl awarie</button>
      </div>
      <div class="h-full w-px bg-gray-800"></div>
      <div class="bg-orange-600 p-1 shadow-lgA rounded-b-lg flex space-x-2 items-center hover:bg-orange-700">
        <button class="bg-transparent text-white px-4 py-2 focus:outline-none" @click="add_report_show">Dodaj awarie</button>
      </div>
    </div>

    <div v-if="showing" class="mt-4">
      <div v-if="show_value" class="grid gap-4">
        <div v-for="(report, index) in reports" :key="index">
          <div v-if="userHasAccess(report)" class="bg-gray-700 p-4 rounded-md shadow-lg relative z-10">
            <p class="text-white">{{ report.nazwa }}
              <button class="bg-orange-600 text-white rounded px-4 py-2 ml-2 hover:bg-orange-700" @click="hide_report(index)">Wybierz</button>
            </p>
          </div>
        </div>
      </div>

      <div v-else class="bg-gray-700 p-6 rounded-lg shadow-lg relative z-10">
        <p class="text-2xl font-semibold flex items-center text-white">Awaria:&nbsp;&nbsp;
          <span v-if="editingIndex !== report_index">{{ report.nazwa }}</span>
          <input v-if="editingIndex === report_index" v-model="editedReports[report_index].nazwa" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-1">
        </p>

        <p class="text-lg font-semibold flex items-center text-white mt-4">Przyczyna:&nbsp;&nbsp;
          <span v-if="editingIndex !== report_index">{{ report.przyczyna }}</span>
          <input v-if="editingIndex === report_index" v-model="editedReports[report_index].przyczyna" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-1">
        </p>

        <p class="text-lg font-semibold flex items-center text-white mt-4">Opis:&nbsp;&nbsp;
          <span v-if="editingIndex !== report_index">{{ report.opis }}</span>
          <input v-if="editingIndex === report_index" v-model="editedReports[report_index].opis" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-1">
        </p>

        <p class="text-lg font-semibold flex items-center text-white mt-4">Miejsce:&nbsp;&nbsp;
          <span v-if="editingIndex !== report_index">{{ report.miejsce }}</span>
          <input v-if="editingIndex === report_index" v-model="editedReports[report_index].miejsce" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-1">
        </p>

        <div>
          <!-- Przycisk do rozwijania/zwinania sekcji komentarzy -->
          <button @click="toggleComments" class="bg-orange-600 text-white rounded px-4 py-2 hover:bg-orange-700">
            {{ isCommentsVisible ? 'Ukryj komentarze' : 'Pokaż komentarze' }}
          </button>
        </div>

        <!-- Sekcja komentarzy, widoczna tylko gdy isCommentsVisible jest true -->
        <div v-if="isCommentsVisible">
          <div v-for="(komentarz, index) in report.komentarze" :key="index" class="bg-gray-600 p-4 rounded-md shadow-lg relative z-10 mt-4">
            <p class="flex items-center text-white">Komentarz:&nbsp;&nbsp;
              <span v-if="editingComment !== index">
                <span v-if="!this.temp_edit_comments[index]">{{ komentarz }}</span>
                <span v-else>
                  <span v-if="(komentarz == this.temp_edit_comments[index].komentarz || this.temp_edit_comments[index].komentarz == 'null') && this.temp_edit_comments[index].option != 'delete'">{{ komentarz }}</span>
                  <span class="text-blue-400" v-if="komentarz != this.temp_edit_comments[index].komentarz && this.temp_edit_comments[index].komentarz != 'null' && this.temp_edit_comments[index].option == 'add'">{{ this.temp_edit_comments[index].komentarz }} <q class="italic text-sm no-quotes">nie zapisano</q></span>
                  <span class="text-red-400" v-if="this.temp_edit_comments[index].komentarz != 'null' && this.temp_edit_comments[index].option == 'delete'">{{ this.temp_edit_comments[index].komentarz }} <q class="italic text-sm no-quotes text-red-400 line-through">komentarz do usunięcia</q></span>
                </span>
              </span>
              <input v-if="editingComment === index" v-model="editComment" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-1">
            </p>
            <div class="flex space-x-2 mt-2" v-if="editingIndex === report_index && editingComment !== index">
              <button class="bg-orange-600 text-white rounded px-4 py-2 hover:bg-orange-700" @click="edit_comment(index)">Edytuj</button>
            </div>
            <div class="flex space-x-2 mt-2" v-if="editingComment === index">
              <button class="bg-orange-600 text-white rounded px-4 py-2 hover:bg-orange-700" @click="addToQueue(index, 'add')">Zapisz</button>
              <button class="bg-gray-700 text-white rounded px-4 py-2 hover:bg-gray-800" @click="cancelEditComment(index)">Anuluj</button>
              <button class="bg-red-600 text-white rounded px-4 py-2 hover:bg-red-700" @click="addToQueue(index, 'delete')">Usuń</button>
            </div>
          </div>
          <div v-for="(komentarz, index) in this.temp_comments" :key="index" class="bg-gray-600 p-4 rounded-md shadow-lg relative z-10 mt-4">
            <p class="flex items-center text-white">Komentarz:&nbsp;&nbsp;
              <span class="text-red-400">{{ komentarz }} <q class="italic text-sm no-quotes">nie zapisano</q></span>
            </p>
          </div>
          <div>
            <input v-if="editingIndex === report_index" v-model="newComment" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-2" type="text" placeholder="Dodaj nowy komentarz">
            <button v-if="editingIndex === report_index" class="bg-orange-600 text-white rounded px-4 py-2 hover:bg-orange-700" @click="addComment">Dodaj komentarz</button>
          </div>
        </div>
        <p class="mt-2 text-white">Właściciel:&nbsp;&nbsp;{{ report.owner.username }}</p>
        <p class="text-white">Kontakt (gmail):&nbsp;&nbsp;{{ report.owner.gmail }}</p>
        <div class="flex space-x-2 mt-4 flex-wrap">
          <button v-if="editingIndex !== report_index" class="bg-orange-600 text-white rounded px-4 py-2 hover:bg-orange-700" @click="edit_report(report_index)">Edytuj</button>
          <button v-if="editingIndex === report_index" class="bg-orange-600 text-white rounded px-4 py-2 hover:bg-orange-700" @click="saveEdit(report_index)">Zapisz</button>
          <button v-if="editingIndex === report_index" class="bg-gray-600 text-white rounded px-4 py-2 hover:bg-gray-800" @click="declineEdit">Anuluj</button>
          <button v-if="editingIndex === report_index" class="bg-red-600 text-white rounded px-4 py-2 hover:bg-red-700" @click="delete_report(report_index)">Usuń</button>
        </div>
      </div>
    </div>

    <div v-if="adding" class="mt-4">
      <div class="bg-gray-700 p-6 rounded-lg shadow-lg relative z-10">
        <p class="text-2xl font-semibold flex items-center text-white">Awaria:&nbsp;&nbsp;
          <input v-model="newReport.nazwa" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-1" type="text" placeholder="podaj nazwę awarii">
        </p>

        <p class="text-lg font-semibold flex items-center text-white mt-4">Przyczyna awarii:&nbsp;&nbsp;
          <select v-model="newReport.przyczyna" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-1">
            <option v-for="cause in causes" :value="cause">{{ cause }}</option>
            <option value="inna">inna</option>
          </select>
        </p>

        <div v-if="newReport.przyczyna === 'inna'" class="mt-2">
          <input v-model="newReport.inna_przyczyna" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-2" type="text" placeholder="Podaj przyczynę awarii">
        </div>

        <p class="text-lg font-semibold flex items-center text-white mt-4">Opis:&nbsp;&nbsp;
          <input v-model="newReport.opis" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-1" type="text" placeholder="Podaj opis awarii">
        </p>

        <p class="text-lg font-semibold flex items-center text-white mt-4">Miejsce:&nbsp;&nbsp;
          <input v-model="newReport.miejsce" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-1" type="text" placeholder="Podaj miejsce awarii">
        </p>

        <div v-for="(komentarz, index) in temp_comments" :key="index" class="bg-gray-600 p-4 rounded-md shadow-lg relative z-10 mt-4">
          <p class="flex items-center text-white">Komentarz:&nbsp;&nbsp;
            <span>{{ komentarz }}</span>
          </p>
        </div>

        <div>
          <input v-model="newComment" class="w-full px-3 py-2 border border-gray-500 rounded-md bg-gray-700 text-white focus:outline-none focus:border-orange-500 mt-2" type="text" placeholder="Dodaj nowy komentarz">
          <button class="bg-orange-600 text-white rounded px-4 py-2 hover:bg-orange-700 mt-2" @click="addComment">Dodaj komentarz</button>
        </div>
        <button class="bg-orange-600 text-white rounded px-4 py-2 mt-2 hover:bg-orange-700" @click="add_report">Zgłoś</button>
      </div>
    </div>
  </div>
</template>

<style>
  .shadow-lgA {
    box-shadow: 0 -7px 6px -1px rgba(255, 165, 0, 0.1), 0 -2px 4px -1px rgba(255, 165, 0, 0.06);
  }
  .shadow-lgA:hover {
    box-shadow: 0 -7px 6px -1px rgba(255, 99, 71, 0.1), 0 -2px 4px -1px rgba(255, 99, 71, 0.06);
  }
</style>

  <script>
  import { dzien2_backend } from 'declarations/dzien2_backend/index';
  
  class Awaria {
      constructor(nazwa, komentarze = [], owner, przyczyna, opis, miejsce) {
          this.nazwa = nazwa;
          this.komentarze = komentarze;
          this.owner = owner;
          this.przyczyna = przyczyna;
          this.opis = opis;
          this.miejsce = miejsce;
      }
  }

  class Item {
    constructor(index, komentarz, option) {
        this.index = index;
        this.komentarz = komentarz;
        this.option = option;
    }
  }
  
  export default {
      data() {
          return {
              isCommentsVisible: false,
              newReport: {
                nazwa: "",
                przyczyna: "",
                inna_przyczyna: "",
                opis: "",
                miejsce: ""
              },
              show_value: false,
              adding: false,
              showing: false,
              reports: [],
              report: new Awaria("", this.$parent.user),
              editingIndex: -1,
              editingComment: -1,
              editedReports: [],
              temp_comments: [],
              temp_edit_comments: [],
              newComment: "",
              editComment: "",
              report_index: -1,
              causes: [
                "błąd projektowy",
                "wada produkcyjna",
                "wada materiału",
                "niewłaściwa eksploatacja",
                "zużycie, zestarzenie",
                "wyjątkowe warunki otoczenia"
              ]
          };
      },
      methods: {
          async show_report() {
              this.temp_edit_comments = [];
              this.report_index = -1;
              this.showing = true;
              this.adding = false;
              await this.fetchReports();
              this.show_value = true;
          },
          async hide_report(index) {
              this.temp_edit_comments = [];
              this.report_index = index;
              this.show_value = false;
              this.report = this.reports[index];
              this.reports = [];
          },
          async add_report_show() {
              this.temp_edit_comments = [];
              this.showing = false;
              this.adding = true;
              this.temp_comments = [];
              this.editingIndex = -1;
          },
          async delete_report(index) {
              this.temp_edit_comments = [];
              this.report_index = -1;
              await dzien2_backend.usun_awarie(index);
              this.report = new Awaria("", this.$parent.user);
              this.showing = false;
              this.adding = false;
              this.temp_comments = [];
              this.editingIndex = -1;
          },
          async edit_report(index) {
              const rep = {
                nazwa: this.report.nazwa,
                przyczyna: this.report.przyczyna,
                opis: this.report.opis,
                miejsce: this.report.miejsce
              };
              this.editedReports[this.report_index] = rep
              this.temp_comments = [];
              this.editingIndex = index;
          },
          async edit_comment(index) {
              this.editComment = this.report.komentarze[index];
              this.editingComment = index;
          },
          async addComment() {
              this.temp_comments.push(this.newComment);
              this.newComment = "";
          },
          async saveEdit(index) {
              const repVec = [
                this.editedReports[index].nazwa || "",
                this.editedReports[index].przyczyna || "",
                this.editedReports[index].opis || "",
                this.editedReports[index].miejsce || ""
              ];
              await dzien2_backend.edytuj_awarie(index, repVec);
              for (const comment of this.temp_comments) {
                  await dzien2_backend.dodaj_komentarz(index, comment);
              }
              for (const item of this.temp_edit_comments) {
                if (item) {
                  if (item.option == 'add') {
                    await this.saveComment(item.index, item.komentarz)
                  }
                  else if (item.option == 'delete')
                  {
                    await this.deleteComment(item.index)
                  }
                }
                  
              }
              this.temp_edit_comments = [];
              await this.fetchReports();
              this.report = this.reports[this.report_index];
              this.reports = [];
              this.editingComment = -1;
              this.temp_comments = [];
              this.editingIndex = -1;
              this.editComment = "";
          },
          async declineEdit() {
              this.temp_edit_comments = [];
              await this.fetchReports();
              this.report = this.reports[this.report_index];
              this.reports = [];
              this.editingComment = -1;
              this.temp_comments = [];
              this.editingIndex = -1;
              this.editComment = "";
          },
          async add_report() {
              const przyczyna = this.newReport.przyczyna === 'inna' ? this.newReport.inna_przyczyna : this.newReport.przyczyna;
              const index = await dzien2_backend.dodaj_awarie(this.newReport.nazwa, this.$parent.user, przyczyna, this.newReport.opis, this.newReport.miejsce);
              this.newReport = {
                nazwa: "",
                przyczyna: "",
                inna_przyczyna: "",
                opis: "",
                miejsce: ""
              };
              for (const comment of this.temp_comments) {
                  await dzien2_backend.dodaj_komentarz(index, comment);
              }
              this.temp_comments = [];
          },
          async fetchReports() {
              this.reports = await dzien2_backend.odczytaj_awarie();
              this.editedReports = new Array(this.reports.length).fill("");
          },
          userHasAccess(report) {
              return this.$parent.user.username === report.owner.username || this.$parent.user.role === 'admin';
          },
          async saveComment(index, comment) {
              await dzien2_backend.edytuj_komentarz(this.report_index, index, comment);
              //this.editingComment = -1;
              //await this.fetchReports();
          },
          async cancelEditComment(index) {
              this.temp_edit_comments[index] = new Item(-1, "null", "none")
              this.editingComment = -1;
              this.editComment = "";
          },
          async deleteComment(index) {
              await dzien2_backend.usun_komentarz(this.report_index, index);
              //this.editingComment = -1;
              //await this.fetchReports();
          },
          async addToQueue(index, option)
          {
              this.temp_edit_comments[index] = new Item(index, this.editComment, option)
              this.editingComment = -1;
              this.editComment = "";
          },
          toggleComments() {
              this.isCommentsVisible = !this.isCommentsVisible;
          }
      }
  }
  </script>