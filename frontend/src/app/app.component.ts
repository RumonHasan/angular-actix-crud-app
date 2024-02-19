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
    this.pizzas = this.apiService.getData().subscribe((data) => {
      console.log(data.json);
    });
  }
}
