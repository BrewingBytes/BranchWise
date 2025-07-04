import IndexPage from "@/pages/IndexPage.vue";
import ProjectPage from "@/pages/ProjectPage.vue";
import { useProjectStore } from "@/stores/project";
import { createMemoryHistory, createRouter, NavigationGuardNext, RouteLocationNormalized } from "vue-router";

const routes = [
	{
		path: "/",
		component: IndexPage
	},
	{
		path: "/project",
		component: ProjectPage,
		beforeEnter: (_to: RouteLocationNormalized, _from: RouteLocationNormalized, next: NavigationGuardNext) => {
			if (!useProjectStore().getSelectedProject) {
				next("/");
			} else {
				next();
			}
		}
	}
];

export default createRouter({
	history: createMemoryHistory(),
	routes
});
