import { GameSocket } from "$lib/GameSocket";
import { gameState } from "$lib/GameState.svelte";
import type { EmoteDataResponse, GameOverResponse, GuessDataResponse, NewUserResponse, Response, RoomJoinResponse } from "$lib/GameModels";

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

  public skip() {
    if (!gameState.user_id) {
      console.warn('UserId is undefined, cannot carry on');
      return;
    }


    if (!gameState.room_id) {
      console.warn('RoomId is undefined, cannot carry on');
      return;
    }

    this.ws.send({
      command: "skip",
      room_id: gameState.room_id
    });
  }

  public editGame() {
    if (!gameState.user_id) {
      console.warn('UserId is undefined, cannot carry on');
      return;
    }


    if (!gameState.room_id) {
      console.warn('RoomId is undefined, cannot carry on');
      return;
    }

    this.ws.send({
      command: "edit_room",
      game_duration: gameState.expectedDuration,
      room_id: gameState.room_id,
    })
  }

  constructor(uri: string) {
    this.ws = new GameSocket(uri);
    this.ws.addEventListener('new_user', this.onNewUser.bind(this));
    this.ws.addEventListener('room_join', this.onRoomJoin.bind(this));
    this.ws.addEventListener('emote', this.onEmote.bind(this));
    this.ws.addEventListener('game_over', this.onGameOver.bind(this));
    this.ws.addEventListener('guess_response', this.onGuessResponse.bind(this));
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
    if (!gameState.started) {
      gameState.started = true;
      gameState.score = 0;
    }

    gameState.currentEmote = typedresponse.emote;
    gameState.guess = '';
  }

  onGameOver(response: Response) {
    // const _typedresponse = response as GameOverResponse;
    console.log('game over')
    gameState.started = false;
  }

  onGuessResponse(response: Response) {
    const typedresponse = response as GuessDataResponse;
    gameState.currentEmote.matched_chars = typedresponse.matched_chars;
    gameState.score = typedresponse.score;
    gameState.display_wrong = true;
  }
}
