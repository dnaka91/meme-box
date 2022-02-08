import {NgModule} from '@angular/core';
import {CommonModule} from '@angular/common';
import {RouterModule, Routes} from "@angular/router";
import {DashboardOverviewComponent} from './dashboard-overview/dashboard-overview.component';
import {MatExpansionModule} from "@angular/material/expansion";
import {MatButtonModule} from "@angular/material/button";
import {MatIconModule} from "@angular/material/icon";
import {MatCardModule} from "@angular/material/card";
import {TranslocoModule} from "@ngneat/transloco";
import {OpenFeedbackButtonModule} from "../../shared/components/open-feedback-button/open-feedback-button.module";
import {HighlightDirective} from './highlight.directive';
import {LatestTwitchEventsComponent} from './latest-twitch-events/latest-twitch-events.component';
import {LatestActionsComponent} from './latest-actions/latest-actions.component';
import {GuardTypePipe} from "./type-guard.pipe";
import {MatTabsModule} from "@angular/material/tabs";

const routes: Routes = [
  {
    component: DashboardOverviewComponent,
    path: ''
  }
]

@NgModule({
  declarations: [
    DashboardOverviewComponent,
    GuardTypePipe,
    HighlightDirective,
    LatestTwitchEventsComponent,
    LatestActionsComponent
  ],
  imports: [
    CommonModule,
    RouterModule.forChild(routes),
    MatExpansionModule,
    MatButtonModule,
    MatIconModule,
    MatCardModule,
    TranslocoModule,
    OpenFeedbackButtonModule,
    MatTabsModule
  ]
})
export class DashboardModule { }
