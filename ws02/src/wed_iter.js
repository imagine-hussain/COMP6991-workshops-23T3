
const arr = [1, 2, 3, 4, 5, 6, 7, 8];

// runs all teh filters -> return by allocating a new aray
// run all the splice -> return a new array
// run all the map -> return a new array

const res = arr.filter((item) => {
  console.log('filter', item);
  return item > 1;
})
  .map((item) => {
    console.log('map', item);
    return item * 2;
  })
  .splice(0, 2); // only takes the first twdo elements


console.log(res);

