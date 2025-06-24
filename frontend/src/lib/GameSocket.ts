import { type Request, type Response, type ResponsesCommands } from '$lib/GameModels';

export class GameSocket {
  private ws: WebSocket;
  private dispatchMap: Map<ResponsesCommands, Array<(response: Response) => void>>;
  private onConnectCallbacks: Array<() => void>;
  private onDisconnectCallbacks: Array<() => void>;
  private backlog: Array<string>;

  constructor(uri: string, session_token: string) {
    this.ws = new WebSocket(uri);
    this.dispatchMap = new Map();
    this.onConnectCallbacks = [];
    this.onDisconnectCallbacks = [];
    this.backlog = [];

    this.authenticate(session_token);
    this.ws.addEventListener('message', this.onMessage.bind(this));
    this.ws.addEventListener('open', this.onConnect.bind(this));
    this.ws.addEventListener('close', this.onDisconnect.bind(this));
    this.ws.addEventListener('error', this.onError.bind(this));
  }

  authenticate(session_token: string) {
    this.send({
      jwt: session_token
    });
  }

  public addEventListener(event: ResponsesCommands, fn: (response: Response) => void) {
    if (!this.dispatchMap.has(event)) {
      this.dispatchMap.set(event, []);
    }
    this.dispatchMap.get(event)?.push(fn);
  }

  public addConnectionStatusListener(event: 'connect' | 'disconnect', fn: () => void) {
    switch (event) {
      case 'connect':
        this.onConnectCallbacks.push(fn);
        return;
      case 'disconnect':
        this.onDisconnectCallbacks.push(fn);
        return;
      default:
        console.warn('idk what you are even doing shaking my head');
    }
  }

  public send(request: Request) {
    const data = JSON.stringify(request);
    console.log(data);
    if (this.ws.readyState === this.ws.OPEN) {
      this.ws.send(data);
    } else {
      this.backlog.push(data);
    }
  }

  onMessage(ev: MessageEvent) {
    const data = JSON.parse(ev.data) as Response;
    this.dispatchMap.get(data.command)?.forEach((fn) => fn(data));
  }

  onConnect() {
    for (const message of this.backlog) {
      this.ws.send(message);
    }
    this.backlog = [];
    this.onConnectCallbacks.forEach(fn => fn())
  }

  onDisconnect() {
    this.onDisconnectCallbacks.forEach(fn => fn())
  }

  onError(ev: Event) {
    console.error(ev);
  }
}
