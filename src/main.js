import {set_tile, setcheck, setplayer, checkmove, currentplayer, remcheck, reset, getpossible, ischeck, manageattacker, updatepieces} from "./commands.js";

const DefaultPieceMap = {"a1": "WRook", "b1": "WKnight", "c1": "WBishop", "d1": "WQueen", "e1": "WKing", "f1": "WBishop", "g1": "WKnight", "h1": "WRook", "a8": "BRook", "b8": "BKnight", "c8": "BBishop", "d8": "BQueen", "e8": "BKing", "f8": "BBishop", "g8": "BKnight", "h8": "BRook"}
let selectedTile;

console.log("loaded")

function addlog(start, end) {
  const list = document.querySelector(".movelist")
  const empty = document.createElement("br")
  const log = document.createElement("i")
  log.append(start + " -> " + end)
  list.appendChild(empty)
  list.appendChild(log)
}

function checklog(att, tar) {
  const list = document.querySelector(".check")
  const empty = document.createElement("br")
  const log = document.createElement("i")

  log.append(tar + " is in check by " + att)
  list.appendChild(empty)
  list.appendChild(log)
}


/**
 * Moves a piece. Does not check if the move is valid
 * @param {*} start Start position in standard notation eg. "a1"
 * @param {*} end End position in standard notation eg. "a2"
 */
function move_piece(start, end) {

  const board = document.querySelector(".chessboard")
  const starttile = board.querySelector("#" + start)
  const endtile = board.querySelector("#" + end)
  const a = starttile.firstChild.id
  let managedthismove = []

  // manage attackers from the old tile. Need to get all a Queen's targets from the old tile, check if any are enemy pieces,
  // and then calculate all of that pieces moves, and manage them accordingly
  


  updatepieces(start, end, managedthismove).then(() => {

  // Do the rest of the move logic

    console.log(`moving ` + start + " to " + end)
    addlog(start, end)

    
    
    
    // Sets tile you started on to empty
    set_tile(start, "")
    const piece = starttile.firstChild
    // Sets tile you finish on to the piece
    set_tile(end, piece.id)
    
    starttile.removeChild(piece)
    endtile.appendChild(piece)
  
    const list = document.querySelector(".check")
    if (list.childNodes.length > 2) {
      remcheck()
      list.removeChild(list.lastChild)
    }
  

    // updates color/active player
    currentplayer().then(res => {
      if (res == "White") {
        setplayer("Black")
      } else if (res == "Black") {
        setplayer("White")
      }
    })


  }).then(() => {

    // this code needs fixing.

    updatepieces(end, end, managedthismove)


    // this is the check after moving (possible moves from new pos) THIS DOES NOT MANAGE ANY OF THE ATTACKERS
  //   getpossible(end, a).then(r => {
  //     let a = end
  //     r.forEach(move => {
  //       checkmove(a, move).then(x => {
  //         if (x) {
  //           let target = board.querySelector("#" + move)
  //           let current = board.querySelector("#" + a).firstChild.id
  
  //           if (Math.abs(parseInt(a[1]) - parseInt(move[1])) < 3) {
  //             if (!managedthismove.includes(move)) {
  //               console.log("managing " + move + " || " + a)
  //               manageattacker(move)
  //               managedthismove.push(move)
  //             }
  //           }
  
  //           if (target.hasChildNodes()) {
  //             if (target.firstChild.id.includes("King")) {
  //               console.log("check")
  //               setcheck(a, target.id)
  //               checklog(a, target.firstChild.id)
  //             }
  //           }
  //         }
  //       })
  //     })
  //   })
   })
}


function takePiece(start, end) {
  end.innerHTML = ""
  move_piece(start.id, end.id)
  swapcolor(start)
  set_tile(start.id, "")
  //set_tile(end.id, start.firstChild.id)
}


function tileClicked(tile) {
  remove_preview()




  // Check if the tile has a piece on it
  if (tile.hasChildNodes()) {
    if (selectedTile) {
      // Can you take the piece?
      if (tile.firstChild.id[0] !== selectedTile.firstChild.id[0]) {
        // Try to take
        checkmove(selectedTile.id, tile.id).then(res => {
          if (res) {
            takePiece(selectedTile, tile)
          } else {
            swapcolor(selectedTile)
          }
        })
      } else {
        // It is your piece, so change selection or deselect
        if (selectedTile == tile) {
          // deselect
          swapcolor(selectedTile)
          selectedTile = null
        } else {
          // chance selection
          console.log("clicked on diff color")
          swapcolor(selectedTile)
          selectedTile = tile
          swapcolor(selectedTile)
          highlightlegal(tile)
        }
      }
    } else {

      let color = tile.firstChild.id[0]
      currentplayer().then(res => {
        if (color == res[0]) {
          swapcolor(tile)
          highlightlegal(tile)
        }
      })
    }
  } else {
    // New tile doesnt have anything, but you previously had a selected piece (trying to move it)
    if (selectedTile) {
      const movefrom = tile.id
      const moveto = selectedTile.id
      const a = selectedTile

      checkmove(selectedTile.id, tile.id).then(res => {
        if (res) {
          move_piece(moveto, movefrom)
        }
      })

      // Deselect that piece
      selectedTile = null
      swapcolor(a)
     // highlightlegal(tile)
    }
  }
}

function swapcolor(tile) {
  if (selectedTile && selectedTile === tile) {selectedTile = null}
  else if (selectedTile) {selectedTile.click()}

  switch (tile.className) {
    case "black":
      tile.className = "selectedfromblack"
      selectedTile = tile
      break;
    case "white":
      tile.className = "selectedfromwhite"
      selectedTile = tile
      break
    case "selectedfromblack":
      tile.className = "black"
      break
    case "selectedfromwhite":
      tile.className = "white"
      break

    default: break;
  }
}



/**
 * Draws the board ready to have pieces drawn on it
 */
function createBoard() {

  const board = document.querySelector(".chessboard")
  const boardSize = 8
  const files = ["a", "b", "c", "d", "e", "f", "g", "h"]

  let xpos = 8
  let ypos = 0

  for (let i = 0; i < boardSize * boardSize; i++) {
    const square = document.createElement('div');
    
    if ((i + Math.floor(i / boardSize)) % 2 === 0) {
      square.className = 'white';
    } else {
      square.className = 'black';
    }
    
    square.id = files[ypos] + xpos
    square.addEventListener("click", function() {tileClicked(this)})
    board.appendChild(square);
    
    if (ypos == 7) {
      ypos = 0
      xpos--
    } else {
      ypos++
    }

  }


}

export function resetBoard() {
  reset()
  console.log("Reseting...")
  const board = document.querySelector(".chessboard")
  
  let arr = [...board.children]
  
  arr.forEach(i => {
    i.innerHTML = ''
  })
  
  if (selectedTile) {
    swapcolor(selectedTile)
  }

  const info = document.querySelector(".currentplayer")
  if (info.childElementCount > 1) {
    info.removeChild(info.lastChild)
  }
  let x = document.createElement("i")
  currentplayer().then(res => {
    x.textContent = res
    info.appendChild(x)
  })


  arr.forEach(item => {
    let id = item.id
    if (id in DefaultPieceMap) {

      let i = document.createElement("img")
      i.width = 40
      i.height = 40
      let piece = DefaultPieceMap[id]
      i.id = piece
  
      i.src = "/assets/" + piece + ".png"
      item.appendChild(i)

      set_tile(i.parentElement.id, i.id)

    } else if (id.includes("2")) {

      let i = document.createElement("img")
      i.width = 40
      i.height = 40
      i.src = "/assets/WPawn.png"
      i.id = "WPawn"
      item.appendChild(i)

      set_tile(i.parentElement.id, i.id)

    } else if (id.includes("7")) {

      let i = document.createElement("img")
      i.width = 40
      i.height = 40
      i.src = "/assets/BPawn.png"
      i.id = "BPawn"
      item.appendChild(i)

      set_tile(i.parentElement.id, i.id)
    }

  })
}



document.addEventListener("DOMContentLoaded", function () {
  createBoard()
  resetBoard()
})
document.getElementById("resetboard").addEventListener("click", resetBoard)



function remove_preview () {
  // Get all elements with the specified class
var elements = document.getElementsByClassName('legal');

// Convert the HTMLCollection to an array
var elementsArray = Array.from(elements);

// Remove each element from the DOM
elementsArray.forEach(function(element) {
  element.parentNode.removeChild(element);
});

  // Get all elements with the specified class
  var elements2 = document.getElementsByClassName('legaltake');

  // Convert the HTMLCollection to an array
  var elementsArray2 = Array.from(elements2);
  
  // Remove each element from the DOM
  elementsArray2.forEach(function(element) {
    element.parentNode.removeChild(element);
  });
}

async function highlightlegal(tile) {
  let start = performance.now()
    const parentElement = document.querySelector(".chessboard"); 


    getpossible(tile.id, tile.firstChild.id).then(res => {
      res.forEach(x => {
        checkmove(tile.id, x).then(r => {
          if (r) {
            const legaltile = parentElement.querySelector("#" + x)

            let cn = "legal"
            if (legaltile.hasChildNodes()) {
              if (legaltile.firstChild.id[0] != tile.firstChild.id[0]) {
                cn = "legaltake"
              } else {
                cn = "legal"
              }
            } else {
              cn = "legal"
            }
            const l = document.createElement("div")
            l.className = cn
            legaltile.appendChild(l)
          }
        })
      })
    })

  let end = performance.now()
  let time = end - start 
  console.log("Scanned all moves in " + time + "ms")
}