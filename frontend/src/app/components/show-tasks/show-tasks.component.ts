import { Component, OnInit } from '@angular/core';
import { ShowTaskService } from './show-tasks.service';
@Component({
  selector: 'app-show-tasks',
  templateUrl: './show-tasks.component.html',
  styleUrls: ['./show-tasks.component.scss'],
})
export class ShowTasksComponent implements OnInit {
  public pizzas: any = [];
  constructor(private showTaskService: ShowTaskService) {}

  ngOnInit(): void {
    this.showTaskService.getData().subscribe((data) => {
      this.pizzas = data;
      console.log(this.pizzas);
    });
  }

  deletePizza(delete_id: any) {
    return this.showTaskService.deletePizza(delete_id);
  }

  addComment(task_id: any) {
    return this.showTaskService.addCommentToTask(task_id);
  }
}
