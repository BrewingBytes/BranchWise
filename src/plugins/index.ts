import { App } from "vue";

import vuetify from "./vuetify";
import router from "../router";

export default function (app: App) {
    app.use(vuetify).use(router);
}
