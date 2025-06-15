/** Requests */

export type CreateRoomRequest = {
  command: "create_room"
};

export type EditRoomRequest = {
  command: "edit_room",
  room_id: string,
  game_duration: number,
};

export type JoinRoomRequest = {
  command: "join_room",
  room_id: string,
};

export type StartGameRequest = {
  command: "start_game",
  room_id: string
}

export type SubmitGuessRequest = {
  command: "submit_guess",
  room_id: string,
  guess: string,
};

export type Request = CreateRoomRequest | EditRoomRequest | JoinRoomRequest | StartGameRequest | SubmitGuessRequest;

/** Responses */
export type NewUserResponse = {
  command: "new_user",
  user_id: string,
};

export type RoomJoinResponse = {
  command: "room_join",
  room_id: string,
};

export type EmoteDataResponse = {
  command: "emote",
  emote: {
    matched_chars: string,
    url: string,
  }
};

export type GuessDataResponse = {
  command: "guess_response",
  matched_chars: string
};

export type GameStartedResponse = {
  command: "game_started"
};

export type GameOverResponse = {
  command: "game_over",
  // emote: {
  //   name: string,
  //   id: string,
  // }
};

export type ResponsesCommands = "new_user" | "room_join" | "emote" | "guess_response" | "game_started" | "game_over";
export type Response = NewUserResponse | RoomJoinResponse | EmoteDataResponse | GuessDataResponse | GameStartedResponse | GameOverResponse;
