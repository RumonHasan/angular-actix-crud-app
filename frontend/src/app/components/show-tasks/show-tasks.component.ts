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
    });
  }

  deletePizza(delete_id: any) {
    return this.showTaskService.deletePizza(delete_id);
  }
}
