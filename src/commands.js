// This file provides functions that allow the frontent javascript code to run rust functions through tauri's command system. 
// All commands can then be imported to main.js to be used in the program.
// As each command will return a promise, it is important to make use of .then() or similar to obtain data from the backend.
// Individual command documentation will explain returns, parameters and use cases.
// Importing invoke below is required for the commands to work.
const { invoke } = window.__TAURI__.tauri;


/**
 * Sets a certain tile to have a certain piece on it.
 * @param {String} tile The tile to set, in standard notation. (eg. "b3")
 * @param {*} piece The piece to put on the tile, using this modules notation. (eg. "WPawn")
 */
export async function set_tile(tile, piece) {
    await invoke("set_tile", { tile: tile, piece: piece })
}


/**
 * Returns all the possible moves from a certain tile, based on the piece located there.
 * @param {String} tile The tile to reference.
 * @param {String} piece The piece to reference. 
 * @returns {Promise<[String]>} An array containing all of the tiles in standard notation.
 */
export async function getpossible(tile, piece) {
    return await invoke("get_possible_moves", {tile: tile, piece: piece})
}

/**
 * Checks if a move is legal, based on a start and end position.
 * @param {*} start The tile to start on.
 * @param {*} end The tile to end on.
 * @returns {Promise<boolean>} Whether or not a move is legal. 
 */
export async function checkmove(start, end) {
    return await invoke("legal_move_checker", {start: start, end: end})
}


/**
 * Resets the backend board data to default.
 */
export async function reset() {
    await invoke("reset")
}


/**
 * Checks if a move is a check based on if the target tile has a king.
 * @param {String} target The target tile that is being attacked.
 * @returns {Promise<boolean>} Result of check.
 */
export async function ischeck(target) {
    return await invoke("is_check", {target: target})
}


/**
 * Sets an active check move for the backend.
 * @param {*} attacker The tile where the attacking piece is.
 * @param {*} target The tile where the target of the check is.
 */
export async function setcheck(attacker, target) {
    await invoke("set_check", {attacker: attacker, target: target})
}


/**
 * Removes a check from the backend.
 */
export async function remcheck() {
    await invoke("rem_check")
}


/**
 * Gets the current player to move.
 * @returns {Promise<"White" | "Black"} The colour of the player whose turn it currently is.
 */
export async function currentplayer() {
    let res = await invoke("get_current")
    if (res == true) {
        return "White"
    } else {
        return "Black"
    }
}


/**
 * Sets the current player whose turn it is. Sets the backend and frontend.
 * @param {*} player The player to set the turn to.
 */
export async function setplayer(player) {
    let r;
    if (player == "White") {
        r = true
    } else if (player == "Black") {
        r = false
    }
    await invoke("set_current", {new: r})

    const info = document.querySelector(".currentplayer")
    info.removeChild(info.lastChild)
    let x = document.createElement("i")
    x.textContent = player
    info.appendChild(x)
}


/**
 * Handles an attacked tile.
 * @param {String} tile Tile to add/remove as an attacker.
 */
export async function manageattacker(tile) {
    await invoke("manage_attacker", {tile: tile})
}


export async function updatepieces(start, end, managedthismove) {
    currentplayer().then(player => {
        let q = player[0] == "W" ? "B" : "W"
        let fakeq = q + "Queen"
        // gets all the tiles that a queen from the old tile could've seen (legal or not)
        getpossible(start, fakeq).then(tiles => {
          let oldtile = start
          tiles.forEach(tile => {
            // if this check is true, that means the tile we are looking at has a piece on it, which can then be managed. Also checks we arent updating the new.
            if (document.querySelector("#" + tile).hasChildNodes() && tile !== end) {
              console.log("tile to manage: " + tile)
    
              // From here, we manage this tile by rechecking its moves to see if they have been updated.
    
              // get all the possible moves from the tile to find any new tiles it can see
              getpossible(tile, document.querySelector("#" + tile).firstChild.id).then(possible => {
                possible.forEach(destination => {
                  // Validate if the move is actually legal
                  checkmove(tile, destination).then(legal => {
                    // If move is legal, continue update and manage the move. Pawn moves causing problems
                    if (legal) {
                        if (!managedthismove.includes(destination)) {
                          console.log("handling " + destination)
                          manageattacker(destination)
                          managedthismove.push(destination)
                        }
                      }
                  })
                })
              })
            }
          })
        })
    })
}