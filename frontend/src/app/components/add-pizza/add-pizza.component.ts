import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { HttpClient } from '@angular/common/http';
import { ActivatedRoute, Router } from '@angular/router';
import { MatSnackBar } from '@angular/material/snack-bar';

@Component({
  selector: 'app-add-pizza',
  templateUrl: './add-pizza.component.html',
  styleUrls: ['./add-pizza.component.scss'],
})
export class AddPizzaComponent implements OnInit {
  public myForm = new FormGroup({
    inputField: new FormControl(''),
  });
  public pizza_name: any = 'Test Pizza';

  constructor(
    private http: HttpClient,
    private activeRoute: ActivatedRoute,
    private router: Router,
    private snackbar: MatSnackBar
  ) {}

  ngOnInit(): void {}

  // submission and switching back to routes
  onSubmit() {
    if (this.pizza_name) {
      console.log(this.pizza_name);
      return this.http
        .post<any>(`http://127.0.0.1:8080/buy`, {
          pizza_name: this.pizza_name,
        })
        .subscribe((data) => {
          if (data) {
            const { created_pizza, created_message } = data;
            this.snackbar.open(created_pizza.pizza_name, created_message, {
              duration: 3000,
            });
            // switching back to show pizza when pizza has been created
            this.router.navigate(['/show'], {
              relativeTo: this.activeRoute,
            });
          }
        });
    }
    return;
  }
}
