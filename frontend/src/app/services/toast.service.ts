import { Injectable } from '@angular/core';
import { EMPTY, Subject } from 'rxjs';

export class Toast {
  id: number;
  type: ToastType;
  message: string;

  constructor(type: ToastType, message: string) {
    this.id = new Date().getTime();
    this.type = type;
    this.message = message;
  }
}

export enum ToastType {
  Success = 'success',
  Error = 'error',
}

@Injectable({
  providedIn: 'root',
})
export class ToastService {
  toast$: Subject<Toast> = new Subject<Toast>();

  success(message: string) {
    this.toast$.next(new Toast(ToastType.Success, message));
    return EMPTY;
  }

  error(message: string) {
    this.toast$.next(new Toast(ToastType.Error, message));
    return EMPTY;
  }
}
