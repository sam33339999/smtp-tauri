<script setup>
import { ref } from 'vue';
import { nanoid } from 'nanoid';
import SmtpForm from "./components/SmtpForm.vue";
import SmtpContent from "./components/SmtpContent.vue";
import { invoke } from "@tauri-apps/api/tauri";
import Button from 'primevue/button';
import Divider from 'primevue/divider';
import Message from 'primevue/message';
import Panel from 'primevue/panel';
import ProgressBar from 'primevue/progressbar';

const config = ref({host: "127.0.0.1", port: 25, username: "", password: "", security: "None"});
const content = ref({from: "", to: "", subject: "主旨", html: ""});
const retMessages = ref([]);
const pendingMessages = ref([]);

const templates = [
  {label: "basic(25)", host: "127.0.0.1", port: 25, security: "None"},
  {label: "Mailpit", host: "127.0.0.1", port: 1025, security: "None"},
  {label: "Gmail", host: "smtp.gmail.com", port: 587, security: "StartTLS"},
  {label: "AWS(northeast-1)", host: "email-smtp.ap-northeast-1.amazonaws.com", port: 587, security: "StartTLS"},
];

// 清除等待處理的消息 by identify
const dropPendingMessage = (identify) => {
  pendingMessages.value = pendingMessages.value.filter((msg) => msg.identify !== identify);
};

const sendMail = async () => {
  const identify = nanoid();
  try {
    pendingMessages.value = [{ identify, type: 'info' }, ...pendingMessages.value];

    const result = await invoke("send_mail", {
      smtp: {
        config: {
          host: config.value.host,
          port: parseInt(config.value.port),
          username: config.value.username === "" ? null : config.value.username,
          password: config.value.password === "" ? null : config.value.password,
          security: config.value.security,
        },
        content: content.value,
      }
    });

    let msg = {
      type: "success",
      result: result,
    };

    retMessages.value = [msg, ...retMessages.value];
    dropPendingMessage(identify);
  } catch (e) {
    let msg = {
      type: "error",
      result: e,
    };
    console.error(e);
    retMessages.value = [msg, ...retMessages.value];
    dropPendingMessage(identify);
  }
};

const cleanAll = () => {
  retMessages.value = [];
  pendingMessages.value = [];
};

</script>

<template>
  <div class="m-2 h-64 mt-4 flex">
    <div class="mx-2" style="width: 25%">
      <SmtpForm :hasTemplates="templates" label="配置" v-model:setting="config"></SmtpForm>
      <Divider />

      <div class="mt-10 w-full flex justify-around ">
        <Button label="寄送(Send)" @click="sendMail"></Button>
        <Button label="清除(Clean)" @click="cleanAll" severity="danger"></Button>
      </div>
      <Divider />
      <div class="mt-5 w-full max-h-96 overflow-scroll h-full ">
        <div class="mt-2" v-for="(msg, idx) of pendingMessages" :key="`pending-${idx}`">
          <Message>{{ msg }}</Message>
        </div>
        <div class="mt-2" v-for="(msg, idx) of retMessages" :key="`ret-${idx}`">
          <Message :severity="msg.type">{{ msg.result }}</Message>
        </div>
      </div>
    </div>

    <div class="mx-2" style="width: 75%">
      <SmtpContent label="信件內容" v-model:form="content"></SmtpContent>
      <ProgressBar :value="pendingMessages.length"/>
    </div>

    <!-- <div class="mx-2 h-full min-h-max" style="width: 10%">
      <Panel header="pending" class="h-full bg-red-200">
        
      </Panel>
    </div> -->
  </div>
</template>

<style scoped>

</style>
