import { GameSocket } from "$lib/GameSocket";
import { gameState } from "$lib/GameState.svelte";
import type { EmoteDataResponse, NewUserResponse, Response, RoomJoinResponse } from "$lib/GameModels";

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

    this.ws.send({
      command: "join_room",
      room_id
    })
  }

  public submitGuess(guess: string) {
    if (!gameState.user_id) {
      console.warn('UserId is undefined, cannot carry on');
      return;
    }


    if (!gameState.room_id) {
      console.warn('RoomId is undefined, cannot carry on');
      return;
    }

    this.ws.send({
      command: "submit_guess",
      room_id: gameState.room_id,
      guess
    })
  }

  constructor(uri: string) {
    this.ws = new GameSocket(uri);
    this.ws.addEventListener('new_user', this.onNewUser.bind(this));
    this.ws.addEventListener('room_join', this.onRoomJoin.bind(this));
    this.ws.addEventListener('emote', this.onEmote.bind(this));
  }

  onNewUser(response: Response) {
    const typedresponse = response as NewUserResponse;
    gameState.user_id = typedresponse.user_id;
  }

  onRoomJoin(response: Response) {
    const typedresponse = response as RoomJoinResponse;
    gameState.room_id = typedresponse.room_id;
  }

  onEmote(response: Response) {
    const typedresponse = response as EmoteDataResponse;
    gameState.started = true;
    gameState.currentEmote = typedresponse.emote;
    gameState.guess = '';
  }
}
