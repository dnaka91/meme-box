import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {CustomPortSettingComponent} from "./custom-port-setting.component";
import {MatIconModule} from "@angular/material/icon";
import {MatFormFieldModule} from "@angular/material/form-field";
import {ReactiveFormsModule} from "@angular/forms";
import {MatInputModule} from "@angular/material/input";
import {MatButtonModule} from "@angular/material/button";
import {ConfigCardModule} from "../config-card/config-card.module";


@NgModule({
  declarations: [CustomPortSettingComponent],
  exports: [
    CustomPortSettingComponent
  ],
  imports: [
    CommonModule,
    MatIconModule,
    MatFormFieldModule,
    ReactiveFormsModule,
    MatInputModule,
    MatButtonModule,
    ConfigCardModule
  ]
})
export class CustomPortSettingModule { }
