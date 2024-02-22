import { Component, OnInit } from '@angular/core';
import { AppService } from './app.component.service';
@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss'],
})
export class AppComponent implements OnInit {
  constructor(private apiService: AppService) {}

  ngOnInit(): void {}
}
