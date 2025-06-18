/** Requests */

export type Authenticate = {
  jwt: string;
};

export type CreateRoomRequest = {
  command: 'create_room';
};

export type EditRoomRequest = {
  command: 'edit_room';
  room_id: string;
  game_duration: number;
};

export type JoinRoomRequest = {
  command: 'join_room';
  room_id: string;
};

export type StartGameRequest = {
  command: 'start_game';
  room_id: string;
};

export type SubmitGuessRequest = {
  command: 'submit_guess';
  room_id: string;
  guess: string;
};

export type SkipRequest = {
  command: 'skip';
  room_id: string;
};

export type Request =
  | CreateRoomRequest
  | EditRoomRequest
  | JoinRoomRequest
  | StartGameRequest
  | SubmitGuessRequest
  | SkipRequest
  | Authenticate;

/** Responses */
export type ErrorResponse = {
  command: 'error';
  error_type: ErrorTypes;
  error_msg: string;
};

export type NewUserResponse = {
  command: 'new_user';
  user_id: string;
};

export type RoomJoinResponse = {
  command: 'room_join';
  room_id: string;
};

export type EmoteDataResponse = {
  command: 'emote';
  emote: {
    matched_chars: string;
    url: string;
  };
};

export type GuessDataResponse = {
  command: 'guess_response';
  matched_chars: string;
  score: number;
};

export type GameStartedResponse = {
  command: 'game_started';
};

export type GameOverResponse = {
  command: 'game_over';
  new_room_id: string;
  // emote: {
  //   name: string,
  //   id: string,
  // }
};

export type GameUpdateResponse = {
  command: 'game_update';
  scores: { [uid: string]: number }[];
};

export type ResponsesCommands =
  | 'new_user'
  | 'room_join'
  | 'emote'
  | 'guess_response'
  | 'game_started'
  | 'game_over'
  | 'game_update'
  | 'error';
export type Response =
  | NewUserResponse
  | RoomJoinResponse
  | EmoteDataResponse
  | GuessDataResponse
  | GameStartedResponse
  | GameOverResponse
  | GameUpdateResponse
  | ErrorResponse;

export type ErrorTypes = 'auth_failed';
