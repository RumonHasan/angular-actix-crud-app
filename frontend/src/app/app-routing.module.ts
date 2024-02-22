import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AppComponent } from './app.component';
import { AddPizzaComponent } from './components/add-pizza/add-pizza.component';
import { ShowTasksComponent } from './components/show-tasks/show-tasks.component';
import { HomeComponentComponent } from './components/home-component/home-component.component';

const routes: Routes = [
  { path: '', component: HomeComponentComponent },
  {
    path: 'add',
    component: AddPizzaComponent,
  },
  {
    path: 'show',
    component: ShowTasksComponent,
  },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
