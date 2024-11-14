import { App } from "vue";

import router from "@/router";
import pinia from "@/plugins/pinia";
import vuetify from "@/plugins/vuetify";

export default function (app: App) {
    app.use(vuetify).use(router).use(pinia);
}
