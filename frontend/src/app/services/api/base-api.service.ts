import { HttpClient, HttpErrorResponse, HttpHeaders, HttpParams } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { catchError, map, Observable } from 'rxjs';
import { environment } from 'src/environments/environment';

interface Response<T> {
  success: boolean;
  result: T;
  error: string;
}

@Injectable({
  providedIn: 'root',
})
export class BaseApiService {
  dateRegex = new RegExp(
    '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}([.][0-9]+)?Z?(\\+[0-9]{2}:[0-9]{2})?$',
  );

  constructor(private httpClient: HttpClient) {}

  get = <T>(url: string, parameters = {}): Observable<T> => this.request('get', url, parameters);

  post = <T>(url: string, body = {}): Observable<T> => this.request('post', url, {}, body);

  put = <T>(url: string, parameters = {}, body = {}): Observable<T> => this.request('put', url, parameters, body);

  delete = <T>(url: string): Observable<T> => this.request('delete', url);

  request<T>(method: string, url: string, parameters = {}, body = {}): Observable<T> {
    const httpOptions = {
      body,
      params: new HttpParams({ fromObject: this.parseParameters(parameters) }),
      headers: new HttpHeaders({ 'Content-Type': 'application/json' }),
      withCredentials: true,
    };

    return this.httpClient.request<Response<T>>(method, environment.baseApi + url, httpOptions).pipe(
      catchError((error: HttpErrorResponse) => {
        throw new Error(error.status === 400 ? 'En uforventet feil har oppstått' : 'Fikk ikke kontakt med serveren');
      }),
      map((response: Response<T>) => {
        if (response.success) return this.parseResponse<T>(response.result);
        throw new Error('En uforventet feil har oppstått');
      }),
    );
  }

  parseParameters(params: any) {
    for (const [key, value] of Object.entries(params)) {
      if (value === null || value === undefined) {
        delete params[key];
      } else if (value instanceof Date) {
        params[key] = value.toISOString();
      }
    }

    return params;
  }

  parseResponse<T>(response: T) {
    if (response instanceof Array) {
      for (const item of response) {
        this.parseResponse(item);
      }
    } else if (response instanceof Object) {
      for (const [key, value] of Object.entries(response)) {
        if (value instanceof Object) {
          this.parseResponse(value);
        } else if (typeof value === 'string' && this.dateRegex.test(value)) {
          (response as any)[key] = new Date(value);
        }
      }
    }

    return response;
  }
}
