import { GameSocket } from "$lib/GameSocket";
import { gameState } from "$lib/GameState.svelte";
import type { EmoteDataResponse, NewUserResponse, Response, RoomCreateResponse } from "$lib/GameModels";

export class Game {
  private ws: GameSocket;

  public createRoom() {
    if (!gameState.user_id) {
      console.warn('UserId is undefined, cannot carry on');
      return;
    }

    this.ws.send({
      command: "create_room",
    });
  }

  public startGame() {
    if (!gameState.user_id) {
      console.warn('UserId is undefined, cannot carry on');
      return;
    }

    if (!gameState.room_id) {
      console.warn('RoomId is undefined, cannot carry on');
      return;
    }

    this.ws.send({
      command: "start_game",
      room_id: gameState.room_id,
    })
  }

  public joinGame(room_id: string) {
    if (!gameState.user_id) {
      console.warn('UserId is undefined, cannot carry on');
      return;
    }

    // TODO: joining of a room needs to be done by the backend for it to make any sense
    gameState.room_id = room_id;

    this.ws.send({
      command: "join_room",
      room_id
    })
  }

  constructor(uri: string) {
    this.ws = new GameSocket(uri);
    this.ws.addEventListener('new_user', this.onNewUser.bind(this));
    this.ws.addEventListener('room_create', this.onRoomCreate.bind(this));
    this.ws.addEventListener('emote', this.onEmote.bind(this));
  }

  onNewUser(response: Response) {
    const typedresponse = response as NewUserResponse;
    gameState.user_id = typedresponse.user_id;
  }

  onRoomCreate(response: Response) {
    const typedresponse = response as RoomCreateResponse;
    gameState.room_id = typedresponse.room_id;
  }

  onEmote(response: Response) {
    const typedresponse = response as EmoteDataResponse;
    gameState.started = true;
    gameState.currentEmote = typedresponse.emote;
  }
}
