[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-22041afd0340ce965d47ae6ef1cefeee28c7c493a6346c4f15d667ab976d596c.svg)](https://classroom.github.com/a/dOlvi4vh)
# assignment_2

# Rubric for Rubik’s Cube Simulation and Scope Concepts Assignment


# Submission Instructions

This assignment will be submitted through GitHub Classroom. Follow the steps below to complete and submit your work:

## Repository Setup
1. Accept the GitHub Classroom invitation on Canvas and create your own repository.
2. Once your repository is created, clone it to your local machine using the following command:
   ```bash
   git clone https://classroom.github.com/a/dOlvi4vh
   ```

## Assignment Structure

Your repository should have the following structure:

```
/root
  ├── README.md           # Assignment details
  ├── answers.txt         # Answers to Questions 1 and 2 in text format
  ├── SudokuSolver.java   # OR SudokuSolver.py OR SudokuSolver.cpp (based on your language preference)
  ├── RubiksCube.java     # OR RubiksCube.py OR RubiksCube.cpp (based on your language preference)
  └── other project files as necessary
```

## Tasks Breakdown:

### 1. Questions 1 and 2 (Text Format)
- Create a file named `answers.txt`.
- In this file, provide the answers to:
  - **Question 1**: Parse tree and leftmost derivation.
  - **Question 2**: Scope concepts (static scope, dynamic scope with deep binding, dynamic scope with shallow binding).
- Use plain text formatting for clarity and include the explanations.

### 2. Question 3: Sudoku Solver (Coding Assignment)
- Implement your solution for the **Sudoku Solver** in your preferred language (Java, Python, or C++).
- The filename should be one of the following based on your language:
  - `SudokuSolver.java` for Java
  - `SudokuSolver.py` for Python
  - `SudokuSolver.cpp` for C++
- Make sure your code is well-documented and includes comments where necessary.
- Your program should read a predefined Sudoku board and output a valid solution.

### 3. Question 4: Rubik’s Cube Simulation (Coding Assignment)
- Implement your data structure and functions to simulate a Rubik's Cube in your preferred language.
- The filename should be one of the following based on your language:
  - `RubiksCube.java` for Java
  - `RubiksCube.py` for Python
  - `RubiksCube.cpp` for C++
- Your code should allow for flexible cube sizes and implement the logic for rotating layers.
- Be sure to include comments and explanations for how your data structure works and how rotations are handled.

## Submission Steps:
1. Once you've completed the assignment, add your files to your repository:
   ```bash
   git add answers.txt SudokuSolver.* RubiksCube.*
   ```
2. Commit your changes with a meaningful message:
   ```bash
   git commit -m "Completed Rubik's Cube and Sudoku Solver Assignment"
   ```
3. Push your changes to the GitHub repository:
   ```bash
   git push origin main
   ```
4. Ensure that all your files have been successfully uploaded to GitHub.

## Important Notes:
- **Due Date**: Ensure you submit your repository before the deadline specified.
- **Plagiarism**: Your submission must be your own work. Any form of copying from other students or external sources will result in penalties.
- **Code Quality**: Ensure your code is clean, well-structured, and follows standard conventions of your chosen programming language.


## Criteria

| Criteria                         | Excellent (10-9)                          | Good (8-7)                               | Satisfactory (6-5)                        | Needs Improvement (4-3)                 | Unsatisfactory (2-0)                     | Score  |
|----------------------------------|-------------------------------------------|------------------------------------------|-------------------------------------------|------------------------------------------|------------------------------------------|--------|
| **Data Structure Design**        | Data structure is well-defined, flexible, and accounts for cubes of any size; efficient storage in memory. | Data structure is mostly well-defined, supports cubes of any size, and shows good memory use. | Data structure is functional but lacks flexibility or efficiency in some cases. | Data structure is incomplete or inefficient, showing little consideration for flexibility or memory usage. | Data structure is missing or inappropriate for the task. | /10    |
| **Layer Rotation Logic**         | Clear and accurate implementation of layer rotation with proper effects on adjacent faces. | Implementation is mostly accurate but may have minor errors in layer rotation logic or adjacent face handling. | Layer rotation is implemented but contains significant errors in adjacent face handling. | Layer rotation is incomplete or does not affect adjacent faces correctly. | No layer rotation logic implemented. | /10    |
| **Explanation of Scoping Rules** | Thorough, accurate explanation of static and dynamic scoping rules and their effects on output. | Good explanation of static and dynamic scoping with minor inaccuracies or missing details. | Explanation is provided but contains major inaccuracies or is lacking in clarity. | Partial explanation of scoping rules with little clarity or detail. | No explanation of scoping rules. | /10    |
| **Pseudocode Implementation**    | Pseudocode is clear, logically sound, and covers all relevant scenarios. | Pseudocode is mostly correct, with minor logic issues. | Pseudocode works but lacks clarity or contains significant logical errors. | Pseudocode is incomplete or unclear, affecting overall logic. | No pseudocode or entirely incorrect implementation. | /10    |
| **Handling of Cube Size Variations** | Cube size variation is fully supported and implemented effectively. | Cube size variation is mostly supported but may have minor limitations in the implementation. | Cube size variation is considered but not fully implemented or contains major issues. | Cube size variation is mentioned but not effectively supported in the implementation. | No consideration for cube size variation in the implementation. | /10    |
| **Efficiency of the Solution**   | Solution is highly efficient in terms of both memory and computational complexity. | Solution is mostly efficient, with minor areas of improvement for larger cubes. | Solution works but has significant inefficiencies in memory or computation. | Solution is inefficient and may not work well for larger cubes. | Solution is highly inefficient or impractical for the problem. | /10    |
| **Clarity and Code Quality**     | Code is well-organized, easy to read, and follows clear structure and naming conventions. | Code is mostly organized and readable but may have some minor issues in structure or naming. | Code works but is hard to read or follow due to poor organization. | Code is poorly organized and difficult to understand. | Code is incomprehensible or disorganized to the point of being unusable. | /10    |

## Total Score: ___ / 70

---




