import { Observable } from 'rxjs';
import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';

@Injectable({
  providedIn: 'root',
})
export class AppService {
  private apiUrl: any = 'http://127.0.0.1:8080';
  public route_variables: any = {
    pizzas: 'pizzas',
    tasks: 'get_tasks',
    deletePizza: 'delete_pizza',
  };

  constructor(private httpClient: HttpClient, private snackbar: MatSnackBar) {}

  getData(): Observable<any> {
    return this.httpClient.get<any>(
      `${this.apiUrl}/${this.route_variables.pizzas}`
    );
  }

  // deleted item and passing on delete message
  deletePizza(delete_id: any): any {
    return this.httpClient
      .delete<any>(
        `${this.apiUrl}/${this.route_variables.deletePizza}/${delete_id}`
      )
      .subscribe((data) => {
        const { deleted_item, delete_message } = data;
        this.snackbar.open(
          `${deleted_item.pizza_name} has been deleted successfully`,
          delete_message
        );
      });
  }
}
