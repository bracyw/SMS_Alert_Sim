import { APP_INITIALIZER, ApplicationConfig } from "@angular/core";
import { provideRouter } from "@angular/router";

import { routes } from "./app.routes";
import { provideHttpClient, withInterceptors } from "@angular/common/http";
import { EMPTY } from "rxjs";

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes)
  ],
};