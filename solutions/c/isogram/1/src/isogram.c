#include <stdio.h>
#include <ctype.h>

#include "isogram.h"

bool is_isogram(const char phrase[]) {
	
	if (phrase == NULL) return false;
	
	int i, j;
	for (i = 0; phrase[i] != '\0'; i++) {
		
		if (isalpha(phrase[i])) {
			
			// iterate over the phrase to check if the character appears again.
			for (j = 0; phrase[j] != '\0'; j++) {
				
				if (i == j) {
					// The exact same character. Skip it.
					continue;
				}
				
				else if ( tolower(phrase[i]) == tolower(phrase[j]) ) {
					// We found another occurence of the character. Return false.
					return false;
				}
			}
		}
	}
	
	return true;
}