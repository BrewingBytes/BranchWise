import { App } from "vue";

import router from "../router";
import pinia from "./pinia";
import vuetify from "./vuetify";

export default function (app: App) {
    app.use(vuetify).use(router).use(pinia);
}
