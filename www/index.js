// JavaScript source code
// include jquery 1.8.3
var charsPerBox = 1;
// I don't know how to catch the 'keypress event' of an alt-11 or alt-12, because I think they're multiple, so I use < for ♂ and > for ♀
var CODE_CONVERSION_STR = "BCDFGHJKLMNPQRSTAIUEO VWXYZ.,&<>";

function code_convert_single(c) {
  var index = CODE_CONVERSION_STR.indexOf(c);
  if (index == -1) {
    throw("Invalid character: character must be one of " + CODE_CONVERSION_STR);
  }
  return index;
}

// Big note: naive integers are stored as 64 bit floats, but whenever you do a bit operation, they're dropped down to 32 bit _signed_. We aren't doing signed, so >>> 0 is a no-op that tricks JS into treating the number as 32 bit unsigned for future bit operations

function shift_op(letter, field, shift) {    
    if (shift < 32) {        
        // lower half of the options are 4 bits, upper half are 5 bits
        let added_shift = ((letter & 0x10) == 0 ) ? 4 : 5;        
        let letter_mask = (1 << added_shift) - 1; 
        let letter_masked = letter & letter_mask; // not strictly necessary but doesn't hurt
        let saturated_shift = (32 - shift) - added_shift; // this puts the value in its position relative to the bits that are already there (which shift tells us)
        if (saturated_shift < 0) saturated_shift = 0; // manually saturating: this means that the last letter placed in a value is |ed fully at the absolute right, even if some of its bits would flow off the right side (actually a negative left shift)
        let letter_shifted = (letter_masked >>> 0) << (saturated_shift >>> 0); // in OG this was a wrapping shift left, but that should never apply
        console.log(added_shift >>> 0, letter_mask >>> 0, letter_masked >>> 0, saturated_shift >>> 0, letter_shifted >>> 0, field >>> 0, shift >>> 0);
        field = ((field >>> 0) | (letter_shifted >>> 0)) >>> 0; // this puts the bits into the field 
        shift += added_shift;
    }    
    return [field, shift];
  }  

function set_binary_fields(string, container) {
  var arraystr = string.split(""); // this is seriously the 'to array' standard in javascript
  let newStringArray = arraystr.map((item, index) => {
    return code_convert_single(item);
  });
  console.log(newStringArray);

  c1 = 0;
  c1_shift = 0;
  c2 = 0;
  c2_shift = 0;

	newStringArray.forEach( (letter, i) => {
    if (c1_shift < 32) {
        [c1, c1_shift] = shift_op(letter, c1, c1_shift);
        
        // if there would have been overflow in a world where we didn't do a saturated shift, take those overflow bits off the right side and start off c2 and c2_shift with them
        if (c1_shift > 32) { // c1_shift can be no larger than 36, making this a little overkilly
            let bits_to_move = c1_shift - 32;
            let moving_bits_mask = (1 << bits_to_move) - 1;            
            c2 = ((letter & moving_bits_mask) << (32 - bits_to_move)) >>> 0;
            c2_shift = bits_to_move >>> 0;
        }
    } else if (c2_shift < 32) {
        [c2, c2_shift] = shift_op(letter, c2, c2_shift);
    }
 });
 console.log("c1 " + c1.toString(16) + " c2 " + c2.toString(16));
 /// Now set the fields
 console.log(container);
 let id_prefix = container[0].id[0];
 console.log(id_prefix);
 document.getElementById(id_prefix + "_c1_shifted").textContent = (c1 >>> 0).toString(2).padStart(32,0);
 let shifted_c2 = c2 >>> (32-c2_shift);
 document.getElementById(id_prefix + "_c2_shifted").textContent = (shifted_c2 >>> 0).toString(2).padStart(32,0);
 document.getElementById(id_prefix + "_results").textContent = (((c1 >>> 0) ^ (shifted_c2 >>> 0)) >>> 0).toString(2).padStart(32,0);
}

function set_canonical_field(bitelement, canonicalelement) {
  let bits = bitelement.textContent;
  console.log(bits);
  let hexarray = parseInt(bits, 2).toString(16).toUpperCase().split("");
  console.log(hexarray);
  let seed = hexarray.map((x) => {
    let i = parseInt(x,16);
    console.log(i);    
    return CODE_CONVERSION_STR[i];
  });
  console.log(seed);
  canonicalelement.textContent = seed.join("") + "BB";  
}

function compare_output_fields(initial, workspace) {
  let initialStr = initial.textContent;
  let workspaceStr = workspace.textContent;
  let colorizedWorkspace = '<nobr style="display: inline">';
  if (initialStr.length == 0 || workspaceStr.length == 0) return;
  workspaceStr.split("").forEach((bit, i) =>
  {
    if (bit == initialStr[i]) {
      colorizedWorkspace += '<div style="color:green;display: inline">';
    } else {
      colorizedWorkspace += '<div style="color:red;display: inline">';
    }
    colorizedWorkspace += bit + "</div>";    
  });
  colorizedWorkspace += "</nobr>";
  workspace.innerHTML = colorizedWorkspace;
}

function calculate_from_fields(prefix) {
    var txt = document.getElementById(prefix + "0").value +
          document.getElementById(prefix + "1").value + document.getElementById(prefix + "2").value +
          document.getElementById(prefix + "3").value + document.getElementById(prefix + "4").value +
          document.getElementById(prefix + "5").value + document.getElementById(prefix + "6").value +
          document.getElementById(prefix + "7").value + document.getElementById(prefix + "8").value +
          document.getElementById(prefix + "9").value;
    try {
          console.log(txt);
          set_binary_fields(txt, $("#" + prefix + "_rep"));
          set_canonical_field(document.getElementById("i_results"), document.getElementById("i_canonical"))
          compare_output_fields(document.getElementById("i_results"), document.getElementById("w_results"));
    }
    catch (err) {
          console.log(err);
    }
}

function calculate_seed_values(e) {

    if (e.target.value.length != charsPerBox) {
        e.target.value = e.target.value.trim();        
        e.target.value = e.target.value.substr(0,charsPerBox);
        if (e.target.value.length == 0) {
            e.target.value = " ";
        }        
    }
    e.target.value = e.target.value.toUpperCase();    

    if (e.target.value.length == charsPerBox) {
        var t = $( e.target );        
        let prefix = t[0].id[0];
        calculate_from_fields(prefix);                
        if (e.type == "input") t.next().focus();
    }
    
}

function input_handler (e) {
        // Allow: backspace, delete, tab, escape, enter and .
        if ($.inArray(e.keyCode, [46, 8, 9, 27, 13, 110, 190]) !== -1 ||
             // Allow: Ctrl+A
            (e.keyCode == 65 && e.ctrlKey === true) || 
             // Allow: home, end, left, right
            (e.keyCode >= 35 && e.keyCode <= 39)) {
                 // let it happen, don't do anything
                 return;
        }
        // Ensure that it is a valid entry which is everything that isn't referred to here
        //                     not capital letters             and         not &            and    not space       and        not <         and       not >
        if ((e.shiftKey && ((e.keyCode < 65 || e.keyCode > 90) && (e.keyCode != 55))) && ( e.keyCode != 32 ) && (e.keyCode != 188) && (e.keyCode != 190)) {
            e.preventDefault();
        } 
}

$(document).ready(function(){
  $(".i").on("input", calculate_seed_values).keydown(input_handler);
  $(".w").on("input", calculate_seed_values).keydown(input_handler);
  $(".i").on("blur", calculate_seed_values);
  $(".w").on("blur", calculate_seed_values);
  calculate_from_fields("i");
  calculate_from_fields("w");
});