import { Component, OnInit } from '@angular/core';
import { AppService } from './app.component.service';
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit {
  public pizzas: any = [];

  constructor(private apiService: AppService) {}

  ngOnInit(): void {
    this.apiService.getData().subscribe((data) => {
      this.pizzas = data;
    });
  }

  deletePizza(delete_id: any) {
    return this.apiService.deletePizza(delete_id);
  }
}
