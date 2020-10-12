<template>
  <div id="app">
    <div style="text-align: center">
      <div id="title">
        <a to="#">Welcome to Web SSH</a>
      </div>
      <br />

      <label>Server: </label>
      <select style="width: 10%" v-model="SSH" required>
        <option value="10.xx.xx.71:22" selected="selected">10.xx.xx.71</option>
      </select>
      <br />

      <input
        style="width: 10%"
        type="text"
        name="username"
        placeholder="Username"
        v-model="Username"
        required
      />
      <input
        style="width: 10%"
        type="password"
        name="password"
        placeholder="Password"
        v-model="Password"
        required
      />
      <br />

      <div style="display: flex">
        <input
          style="width: 50%"
          type="text"
          name="command"
          placeholder="Please input your command here!"
          v-model="Command"
          required
        />
      </div>
      <br />

      <div class="vld-parent">
        <loading
          :active.sync="isLoading"
          :can-cancel="true"
          :on-cancel="onCancel"
          :is-full-page="fullPage"
          :color="color"
          :loader="loader"
        ></loading>
        <button id="submit" @click="exec" @click.prevent="exec">Execute</button>&nbsp;
        <button id="submit" @click="cleanup" @click.prevent="cleanup">Cleanup</button>
      </div>
      <br />

    </div>
    <code style="white-space: pre-wrap;">
      <text-highlight :queries="queries">{{ Result }}</text-highlight>
    </code>
  </div>
</template>

<script>
// Import component
import Loading from "vue-loading-overlay";
// Import stylesheet
import "vue-loading-overlay/dist/vue-loading.css";
export default {
  name: "app",
  data() {
    return {
      SSH: "",
      Username: "",
      Password: "",
      Command: "",
      Result: "",
      queries: ["ERROR", "WARNING"],
      isLoading: false,
      fullPage: true,
      loader: 'spinner',
      color: '#007bff',
    };
  },
  components: {
    Loading,
  },
  methods: {
    onCancel() {
      console.log("User cancelled the loader.");
    },
    exec() {
      this.isLoading = true;
      let ssh_uri = this.SSH;
      let username = this.Username;
      let password = this.Password;
      let command = this.Command;

      let data = {
        ssh_uri: ssh_uri,
        username: username,
        password: password,
        command: command,
      };
      fetch("http://localhost:8000/ssh", {
        body: JSON.stringify(data),
        headers: {
          "content-type": "application/json",
        },
        method: "POST",
      })
        .then((response) => response.json())
        .then((json) => {
          console.log(json);
          this.Result = json.result;
          this.isLoading = false;
        })
        .catch((e) => {
          console.log(e);
        });
    },
    cleanup() {
      this.isLoading = true;
      this.Result = "";
      setTimeout(() => {
        this.isLoading = false
      }, 250);
    },
  },
};
</script>

<style scoped>
#content {
  width: 250px;
  margin: 0 auto;
  padding-top: 33px;
}
#title {
  padding: 0.5rem 0;
  font-size: 22px;
  font-weight: bold;
  background-color: bisque;
  text-align: center;
}
input[type="text"],
input[type="password"] {
  margin: 6px auto auto;
  width: 250px;
  height: 36px;
  border: none;
  border-bottom: 1px solid #aaa;
  font-size: 16px;
}
#submit {
  margin: 10px 0 20px 0;
  width: 250px;
  height: 33px;
  background-color: bisque;
  border: none;
  border-radius: 2px;
  font-family: "Roboto", sans-serif;
  font-weight: bold;
  transition: 0.1s ease;
  cursor: pointer;
}
input[type="checkbox"] {
  margin-top: 11px;
}
dialog {
  top: 50%;
  width: 80%;
  border: 5px solid rgba(0, 0, 0, 0.3);
}
dialog::backdrop {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.7);
}
#closeDialog {
  display: inline-block;
  border-radius: 3px;
  border: none;
  font-size: 1rem;
  padding: 0.4rem 0.8em;
  background: #eb9816;
  border-bottom: 1px solid #f1b75c;
  color: white;
  font-weight: bold;
  text-align: center;
}
#closeDialog:hover,
#closeDialog:focus {
  opacity: 0.92;
  cursor: pointer;
}
@media only screen and (min-width: 600px) {
  #content {
    margin: 0 auto;
    padding-top: 100px;
  }
}
</style>
