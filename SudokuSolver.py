import copy

# Script for solving a sudoku board. Utilizes a simple elemination method, using a recursive brute force method if that fails.

# Class representing a Sudoku board
# Sudoku boards are represented using dictionarys mapping tuples of positional values mapped to cell values. This was chosen over a two-dimensional array due to not requiring a dedicated "Empty Cell" flag value.
# (Honestly if I was using Rust this would be perfect for the Option<T> enum but alas...)
class SBoard:
    def __init__(self, initial_board):
        self.board = initial_board

    def pos_in_bounds(self, pos):
        for i in pos: 
            if i < 0 or i >= 9: return False
        return True

    # Get the value of the cell at the given x and y position, or Null if out of bounds
    def get_cell(self, pos):
        if not self.pos_in_bounds(pos): return None
        return self.board.get(pos, None)

    # Sets the value of a given cell, prevents setting values out of bounds
    def set_cell(self, pos, value):
        if not self.pos_in_bounds(pos): return None
        self.board[pos] = value

    # Gets a list of all valid cells that still need to be filled
    def get_empty_cells(self):
        out = []
        for x in range(9):
            for y in range(9):
                if not (x, y) in self.board.keys():
                    out.append((x, y))
        return out

    # Returns a set of all cells in the same collum, row, or box as a given cell
    def get_influence_cells(self, pos):
        out = set(()) # A set is used to not have to worry about creating duplicates

        # Get row and collum positions
        for i in range(9):
            out.add((pos[0], i))
            out.add((i, pos[1]))

        box_x = 0
        box_y = 0

        if (pos[0] > 2):
            box_x = 3
        if (pos[0] > 5):
            box_x = 6

        if (pos[1] > 2):
            box_y = 3
        if (pos[1] > 5):
            box_y = 6

        for x in range(3):
            for y in range(3):
                out.add((box_x + x, box_y + y))

        return out

    # Get the set of valid options for the cell's value
    def get_valid_cell_options(self, pos):
        influince_cells = self.get_influence_cells(pos)
        potential_values = map(self.get_cell, list(influince_cells))
        potential_values = filter(lambda x : x != None, potential_values)
        return set(range(1, 10)) - set(potential_values)

    # Prints out the game board with nice formatting
    def print_board(self):
        for y in range(9):
            line = "" 
            for x in range(9):
                val = self.get_cell((x, y))
                if val == None:
                    line += " "
                else:
                    line += str(val)
                if x == 2 or x == 5:
                    line += "|"
            print(line)
            if y == 2 or y == 5:
                print("---+---+---")

    # Debug function, used to print out the cells that influence a given cell
    def print_influince_cells(self, pos):
        cells = self.get_influence_cells(pos)
        for y in range(9):
            line = "" 
            for x in range(9):
                if (x, y) in cells:
                    line += "X"
                else:
                    line += " "
                if x == 2 or x == 5:
                    line += "|"
            print(line)
            if y == 2 or y == 5:
                print("---+---+---")

    # Gets a dictionary mapping positions to sets of potential values
    def get_option_map(self):
        cells = self.get_empty_cells()
        out = {}
        for cell in cells:
            out[cell] = self.get_valid_cell_options(cell)
        return out

    # Run algorithim to solve the board. Prints true if the given board was solvable
    def solve(self):
        while True:
            # Step 1: Get potential values of all cells
            options = self.get_option_map()
            # Step 2: If no potential cells are found, then the board is solved, return true
            if len(options) == 0:
                #print("Solved, no open cells")
                return True
            # Step 3: Loop through cells, if any have only one possible option assign that option. If a cell has no valid options then return False since the puzzle is unsolveable
            found = False
            
            for pos, possibilities in options.items():
                if len(possibilities) == 0:
                    #print("Returning due to impossible board, no option for %s" % str(pos) )
                    #print("Influince cells were %s" % str(self.get_influence_cells(pos)))
                    #print("Possibilities are %s" % str(self.get_valid_cell_options(pos)))
                    #self.print_influince_cells(pos)
                    return False
                if len(possibilities) == 1:
                    found = True
                    self.set_cell(pos, list(possibilities)[0])
            # Step 4: If none of the cells had one valid option, then we have to brute force...
            if found:
                continue
            # Step 4.1: Loop through possible options of first open cell, create a duplicate of self with that cell set, and try to solve
            (pos, possibilities) = options.items()[0];
            for possibility in possibilities:
                other = copy.deepcopy(self)
                other.set_cell(pos, possibility)
                if other.solve():
                    self.board = other.board
                    #print("Returning due to solved board found")
                    return True
            # Step 4.2: If a option gets solved, then the guess was correct, coppy board from sub board and return true
            #print("No option foud through brute force")
            return False
            # Step 4.3: If none of the options returned true, then return False, since board is in an unwinnable state
        return True

# A game board represented as a list of strings
# The NYT Easy Sudoku board on 2024-10-16
pboard = [
    "000867342",
    "432000006",
    "067320005",
    "900073400",
    "200901003",
    "640000010",
    "025710900",
    "100005007",
    "004200580"
]

# Converts to dictionary of cells
items = {}
for y in range(9):
    chars = [x for x in pboard[y]]
    for x in range(9):
        val = int(chars[x])
        if val != 0:
            items[(x, y)] = val

# Solve the board
board = SBoard(items)
board.solve()
board.print_board()
