two_fer <- function(input = NULL) {
  if (is.null(input)) {
    return("One for you, one for me.")
  } else {
    return(paste("One for ", input, ", one for me.", sep = ""))
  }
}