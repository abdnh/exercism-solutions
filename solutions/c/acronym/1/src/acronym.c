#include <ctype.h>
#include <stdlib.h>
#include <string.h>

#include "acronym.h"

char *abbreviate(const char *phrase) {
	
	if (phrase == NULL || strcmp(phrase, "") == 0) return NULL;
	
	int i;
	int words_count = 0;
	int abbreviation_length = 2;
	char *abbreviation = malloc(abbreviation_length * sizeof(char));
	
	for (i = 0; phrase[i] != '\0'; i++) {
		
		if ( ( ( isspace(phrase[i-1]) || phrase[i-1] == '-') && isalpha(phrase[i]) ) || ( i == 0 && isalpha(phrase[i]) ) ) {
			
			if (words_count > abbreviation_length) {
				abbreviation_length *= 2;
				abbreviation = realloc(abbreviation, abbreviation_length * sizeof(char));
			
			}
			abbreviation[words_count] = toupper(phrase[i]);
			words_count ++;
		}
	}
	
	abbreviation[words_count] = '\0';
	return abbreviation;
}