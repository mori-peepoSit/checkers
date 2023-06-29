from util import runtime, memoize

# Default starting position
board2 = [0 for _ in range(64)]
color = 2
white_start = [1, 3, 5, 7, 8, 10, 12, 14, 17, 19, 21, 23]
black_start = [40, 42, 44, 46, 49, 51, 53, 55, 56, 58, 60, 62]
testing_start_1 = [33, 35, 37, 39, 49, 51, 53, 55]
white = [1, 11]
black = [2, 22]

# lookup arrays
col1 = [8,24,40,56]; col2 = [1,17,33,49]
col3 = [10,26,42,58]; col4 = [3,19,35,51]
col5 = [12, 28, 44, 60]; col6 = [5, 21, 37, 53]
col7 = [14,30,46,62]; col8 = [7,23,39,55]

row1 = [1,3,5,7]; row2 = [8,10,12,14]
row3 = [17,19,21,23]; row4 = [24,26,28,30]
row5 = [33,35,37,39]; row6 = [40,42,44,46]
row7 = [49,51,53,55]; row8 = [56, 58, 60, 62]


# Dictionaries
try: 
    print('テスト')
    color_dict = {2: 'Black (黒)', 1:'White (白)'}
    symbol = {0:'〇', 1:'白', 2:'黒', 11:'王', 22:'姫'}
    if input('emote mode?')=='y':
        color_dict = {2: 'Black (🔵)', 1:'White (⚪)'}
        symbol = {0:'〇', 1:'⚪', 2:'🔵', 11:'🟦', 22:'⬜'}
except:
    print('Changed to ascii-mode')
    color_dict = {2: 'Black', 1:'White'}
    symbol = {0:'0', 1:'w', 2:'b', 11:'W', 22:'B'}

change_color_dict = {1:2, 2:1}
scoring_dict = {11: -2, 22:2, 0:0}


# Functions:
def fill_board(w_start, b_start, board):
    '''Fills the board'''
    for i in w_start:
        board[i]=1
    for i in b_start:
        board[i]=2
    return board


board2 = fill_board(white_start, black_start, board2)
# board2[44]=22 # for testing


def cmd_show(board):
    '''displays the board on the console'''
    print('\n')
    for i in range(0,len(board), 8):
        print(f"{i:<2}|{' '.join([symbol[x] for x in board[i:i+8]])}|{i+7}")
    print('\n')
    return


def legal_moves_black(index, board):
    output = []
    if index in row2:
        # Last Rank
        if index not in col1:
            if board[index-9]==0:
                output.append(index-9)
        if index not in col8:
            if board[index-7] == 0:
                output.append(index-7)
        return output # stop early
    

    if index not in col1:
        # Move Up Left
        if board[index-9]==0:
            output.append(index-9)
        # Capture Up Left
        elif index>=16 and index not in col2 and board[index-9] in white and board[index-18]==0:
            output.append(index-18)

    
    if index not in col8:
        # Move Up Right
        if board[index-7]==0:
            output.append(index-7)
        # Capture Up Right
        elif index>=16 and index not in col7 and board[index-7] in white and board[index-14]==0:
                output.append(index-14)

    return output


def legal_moves_white(index, board):
    # top and bottom moving 1 forward fix !
    output = []
    if index in row7:
        if index not in col1:
            if board[index+7]==0:
                output.append(index+7)
        if index not in col8:
            if board[index+9] == 0:
                output.append(index+9)
        # stop early
        return output
    

    if index not in col1:
        # Move Down Left
        if board[index+7]==0:
            output.append(index+7)
        # Capture Down Left
        elif index<=47 and index not in col2 and board[index+7] in black and board[index+14]==0:
            output.append(index+14)


    if index not in col8:
        # Move Down Right
        if board[index+9]==0:
            output.append(index+9)
        # Capture Down Right
        elif index<=47 and index not in col7 and board[index+9] in black and board[index+18]==0:
            output.append(index+18)
        
    return output


def legal_moves_black_king(index, board):
    output = []
    # Moving left
    if index not in col1:
        # Move Up Left
        if index not in row1 and board[index-9]==0:
            output.append(index-9)
        # Capture Up Left
        elif index>=16 and index not in col2 and board[index-9] in white and board[index-18]==0:
            output.append(index-18)
        # Move Down Left
        if index not in row8 and board[index+7]==0:
            output.append(index+7)
        # Capture Down Left
        elif index<=47 and index not in col2 and board[index+7]in white and board[index+14]==0:
            output.append(index+14)

    # Moving right
    if index not in col8:
        # Move Up Right
        if index not in row1 and board[index-7]==0:
            output.append(index-7)
        # Capture Up Right
        elif index>=16 and index not in col7 and board[index-7]in white and board[index-14]==0:
            output.append(index-14)
        # Move Down Right
        if index not in row8 and board[index+9]==0:
            output.append(index+9)
        # Capture Down Right
        elif index<=47 and index not in col7 and board[index+9]in white and board[index+18]==0:
            output.append(index+18)


    return output


def legal_moves_white_king(index, board):
    output = []
    # Moving left
    if index not in col1:
        # Move Up Left
        if index not in row1 and board[index-9]==0:
            output.append(index-9)
        # Capture Up Left
        elif index>=17 and index not in col2 and board[index-9] in black and board[index-18]==0:
            output.append(index-18)
        # Move Down Left
        if index not in row8 and board[index+7]==0:
            output.append(index+7)
        # Capture Down Left
        elif index<=47 and index not in col2 and board[index+7]in black and board[index+14]==0:
            output.append(index+14)

    # Moving right
    if index not in col8:
        # Move Up Right
        if index not in row1 and board[index-7]==0:
            output.append(index-7)
        # Capture Up Right
        elif index>=17 and index not in col7 and board[index-7]in black and board[index-14]==0:
            output.append(index-14)
        # Move Down Right
        if index not in row8 and board[index+9]==0:
            output.append(index+9)
        # Capture Down Right
        elif index<=47 and index not in col7 and board[index+9]in black and board[index+18]==0:
            output.append(index+18)


    return output


##########
def piece_movelist(index, board):
    '''maps pieces to their respective legal move functions'''
    output = []
    match board[index]:
        case 2: 
            temp = legal_moves_black(index, board)
            for i in temp:
                output.append([index, i])
            return output
            #return [index, legal_moves_black(index, board)]
        case 22:
            temp = legal_moves_black_king(index, board)
            for i in temp:
                output.append([index, i])
            return output
        case 1:
            temp = legal_moves_white(index, board)
            for i in temp:
                output.append([index, i])
            return output
        case 11:
            temp = legal_moves_white_king(index, board)
            for i in temp:
                output.append([index, i])
            return output


def movelist(color, board):
    '''iterates through the board and calls piece_movelist()'''
    output = []
    output_clean = []
    for idx,element in enumerate(board):
        if element == color or element==int(str(color)*2):
            temp = piece_movelist(idx, board)
            if temp: output.append(temp)


    # removes empty possible movelists
    for move in output:
        if move:
            for move_2 in move:
                output_clean.append(move_2)
    '''forces to capture if possible
    Returns Captures, Chain (bool)
    '''
    captures = []
    for move in output_clean: # for move in possible moves
        if abs(move[0]-move[1]) >10: # moving 2 squares diagonally is |14| or more
            captures.append(move)
    if captures:
        return captures, True
    return output_clean, False


def make_move(moves, board, auto=False):
    cur = moves[0]
    move = moves[1]
    # black pieces
    new_board = board
    piece = board[cur]
    match move-cur:
        case -18: # capture left
            new_board[cur] = 0; new_board[cur-9]=0; new_board[cur-18]=piece
        case -14: # capture right
            new_board[cur] = 0; new_board[cur-7]=0; new_board[cur-14]=piece
        case -9: # move left
            new_board[cur]=0; new_board[cur-9]=piece
        case -7: # move right
            new_board[cur]=0; new_board[cur-7]=piece
        # white pieces
        case 14: # capture left
            new_board[cur]=0; new_board[cur+7]=0; new_board[cur+14]=piece
        case 18: # capture right
            new_board[cur]=0; new_board[cur+9]=0; new_board[cur+18]=piece
        case 7: # move left
            new_board[cur]=0; new_board[cur+7]=piece
        case 9: # move right
            new_board[cur]=0; new_board[cur+9]=piece
    return new_board


def promote(board):
    for i in row1:
        if board[i] == 2:
            board[i] = 22
            break
    for i in row8:
        if board[i]==1:
            board[i]= 11
            break
    return board


def gameover(board):
    # no black pieces
    if 2 not in board and 22 not in board:
        return True
    # no white pieces
    if 1 not in board and 11 not in board:
        return True
     # no black moves
    if not movelist(2, board): return True
    # no white moves
    if not movelist(1, board): return True
    return False



def score(board, increment=0.1):
    # add outside col reward ?
    score_output = 0 
    # + means favorable for black
    # - means favorable for white
    for idx,square in enumerate(board):
            # testing with i=0.1
        if square == 1:
            # row 1=1.0 + i*0=1.0 (First row)
            # row 7=1.0 + i*7=1.7 (2nd to last row)
            row_nr=(idx)//8
            score_output -= 1 + row_nr*increment
        elif square == 2:
            # row 7=1.0 + i*0=1.0 (First row)
            # row 2=1.0 + i*7=1.7 (2nd to last row)
            row_nr=(idx)//8
            score_output += 1 + (7-row_nr)*increment
        else:
            # Kings and empty squares
            score_output += scoring_dict[square]
    return round(score_output,2)


def make_move_2(move, chain, board, color, show=False):
    '''Given a move, board, color, chain? It returns the board with move or chain of moves made'''
    board = make_move(move, board, True)
    board = promote(board)
    if chain:
        # last move was a capture
        temp,chain = movelist(color, board) 
        if chain and temp[0][0]==move[1]: # same piece?
            make_move_2(temp[0], True, board, color)
    return board


@memoize
def evaluate(cur_color, depth, board, alpha, beta):
    '''Recursively tries out all Games'''
    global c
    # Ending the recursion
    if depth==11:
        return score(board)


    if cur_color == 2:
        # As to not change the original board by reference
        new_board = board[:]
        # positive infinity
        max_eval = -1000
        # generates movelist
        moves,chain = movelist(cur_color, new_board)
        # Ending the recursion (gameover for black)
        if not moves:
            return -100
        # Recurse through all legal moves
        for move in moves:
            c+= 1
            temporary = new_board[:]
            # Make move on copy of board
            new_board = make_move_2(move, chain, new_board, cur_color)
            # recursion
            move_score = evaluate(1, depth+1, new_board, alpha, beta)
            # new best move in current? (Max)
            max_eval= max(move_score,max_eval)
            # new best move in parent? (Max)
            alpha = max(alpha, move_score)
            # pruning
            if beta <= alpha:
                break
            # Undo move
            new_board = temporary[:]
        # Return evaluation
        return max_eval
    

    elif cur_color == 1:
        c+= 1
        # As to not change the original board by reference
        new_board = board[:]
        # negative infinity
        min_eval = 1000
        # placeholder value
        final_move = 0.5
        # generates movelist
        moves,chain = movelist(cur_color, new_board)
        # Ending the recursion (Gameover for White)
        if not moves:
            return 100
        # Dont recurse if theres only 1 Option
        if depth==0 and len(moves)==1:
            return 0
        # Recurse through all legal moves
        for idx,move in enumerate(moves):
            # copy of board
            temporary = new_board[:]
            # make the move on the copy 
            new_board = make_move_2(move, chain, new_board, cur_color)
            # recursion
            move_score = evaluate(2, depth+1, new_board, alpha, beta)
            if depth==0:
                print(f'{idx}, {move}: {move_score}')
            # new best move in current? (Min)
            if move_score < min_eval:
                min_eval = move_score
                if depth == 0:
                    final_move = idx
            # new best move in parent? (Min)
            beta = min(beta, move_score)
            # pruning
            if beta <= alpha:
                break
            # Undo move
            new_board = temporary[:]
        # return move index in depth=0
        if depth == 0:
            return final_move
        # else return evaluation (minimizing player)
        return min_eval




# main loop:
if __name__ == '__main__':
    print('Negative Evaluation means good for White, positive means good for Black')
    while not gameover(board2):
        cmd_show(board2)

        # determine which color is to play
        print(f"{color_dict[color]}'s turn ! (Evaluation: {score(board2)})\n")

        # movelist
        moves, chain = movelist(color, board2)
        # chain returns true if there was a capture

        if not moves: # no legal moves -> gameover
            break

        # input to choose move
        print('Movelist:')
        for c, element in enumerate(moves):
            print(f"{c:<2}",':', element); 
        
        try:
            move_number = int(input('\nChoose a move ! '))
            chosen_move = moves[move_number]
        except:
            move_number = int(input('\nInput a correct number please! '))
        # loop this until its an int
        
        chosen_move = moves[move_number]

        # make move
        board2 = make_move(chosen_move, board2)
        board2 = promote(board2) # Promote if possible
        cmd_show(board2)
        # loop until you can't chain capture anymore
        while chain:
            
            # last move was a capture

            moves, chain = movelist(color, board2)
            chain_captures = [] # to make sure its the same piece, thats capturing again
            for move_i in moves:
                if move_i[0] == chosen_move[1]: # ending pos of last move
                    chain_captures.append(move_i)
            moves = chain_captures

            # is there another capture and is it the same piece?
            if chain and moves:
                print('Movelist:')
                for c, element in enumerate(moves):
                    print(f"{c:<2}",':', element); 
                chosen_move = moves[int(input('\nChoose a move ! '))]

                # make move
                board2 = make_move(chosen_move, board2)
                board2 = promote(board2) # Promote if possible
                cmd_show(board2)
            else:
                chain = False

        # Change color
        color = change_color_dict[color]
    

##########
# Let AI play :
##########
        # Recursive search for best move
        c = 0
        c_move, time = runtime(evaluate, color, 0, board2, -1000, 1000) # returns move number
        print(f'response time: {time} seconds')
        

        # Make best move
        moves, chain = movelist(color,board2)
        try:
            print(f'move: {moves[c_move]} ({c} moves evaluated)')
        except:
            print(f'gameover\n{color_dict[change_color_dict[color]]} wins')
            break
        board2 = make_move_2(moves[c_move], chain, board2, color, True)

        # Change color back to player
        color = 2

# Gameover:
    cmd_show(board2)
    print(f'gameover\n{color_dict[change_color_dict[color]]} wins')