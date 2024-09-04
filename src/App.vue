<script setup>
import { ref } from 'vue';
import SmtpForm from "./components/SmtpForm.vue";
import SmtpContent from "./components/SmtpContent.vue";
import { invoke } from "@tauri-apps/api/tauri";
import Button from 'primevue/button';
import Divider from 'primevue/divider';
import Message from 'primevue/message';

const config = ref({host: "email-smtp.ap-northeast-1.amazonaws.com", port: 587, username: "", password: ""});
const content = ref({from: "", to: "", subject: "測試", text: "???", html: "<h2><Hello World !!!/h2>"});
const retMessages = ref([]);

const sendMail = async () => {
  try {
    console.log(parseInt(config.port));

    const result = await invoke("send_mail", {
      smtp: {
        config: {
          host: config.value.host,
          port: parseInt(config.value.port),
          username: config.value.username,
          password: config.value.password,
        },
        content: content.value,
      }
    });

    let msg = {
      type: "success",
      result: result,
    };

    retMessages.value = [msg, ...retMessages.value];
  } catch (e) {
    let msg = {
      type: "error",
      result: e,
    };
    console.error(e);
    retMessages.value = [msg, ...retMessages.value];
  }
};

const cleanAll = () => {
  retMessages.value = [];
};

</script>

<template>
  <div class="m-2 h-64 mt-4 flex">
    <div class="mx-2" style="width: 30%">
      <SmtpForm v-model:setting="config"></SmtpForm>
      <Divider />

      <div class="mt-10 w-full flex justify-around ">
        <Button label="寄送(Send)" @click="sendMail"></Button>
        <Button label="清除(Clean)" @click="cleanAll" severity="danger"></Button>
      </div>
      <Divider />
      <div class="mt-5 w-full max-h-96 overflow-scroll h-full ">
        <div class="mt-2" v-for="(msg, idx) of retMessages" :key="idx">
          <Message :severity="msg.type">{{ msg.result }}</Message>
        </div>
        
      </div>
    </div>

    <div class="mx-2" style="width: 70%">
      <SmtpContent v-model:form="content"></SmtpContent>
    </div>
  </div>
</template>

<style scoped>

</style>
