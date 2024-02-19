import { Observable } from 'rxjs';
import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';

@Injectable({
  providedIn: 'root',
})
export class AppService {
  private apiUrl: any = 'http://127.0.0.1:8080/pizzas';
  constructor(private httpClient: HttpClient) {}

  getData(): Observable<any> {
    return this.httpClient.get<any>(`${this.apiUrl}`);
  }
}
