# Source files and object files
SRC = src/main.cpp src/logic.cpp
OBJS = $(SRC:.cpp=.o)

# Compiler and flags
CC = g++
CFLAGS = -I./include

# Target names
NAME = oui
DESTDIR = /usr/bin/
RESOURCE = resources/IEEE_OUI.csv
RESOURCEDIR = /usr/share/oui/
MANDIR = /usr/share/man/man1/
MANPAGE = oui.1
SRCMAN = man/
COMPRESSED_MANPAGE = oui.1.gz

# Commands
RM = rm -f
CP = cp -f
MKDIR = mkdir -p
COMPRESS = gzip -c $(SRCMAN)$(MANPAGE) > $(SRCMAN)$(COMPRESSED_MANPAGE)
MANDB = mandb

# Default target
all: $(NAME)

# Build the executable
$(NAME): $(OBJS)
	$(CC) $(CFLAGS) -Wall -Wextra -Wshadow -Wnon-virtual-dtor -pedantic -o $(NAME) $(OBJS) $(LDFLAGS)

# Clean object files
clean:
	$(RM) $(OBJS)

# Clean compressed man page
cleanMan:
	$(RM) $(SRCMAN)$(COMPRESSED_MANPAGE)

# Clean all build artifacts
fclean: clean cleanMan
	$(RM) $(NAME)

# Install the executable and resources
install: $(NAME)
	$(CP) $(NAME) $(DESTDIR)
	$(MKDIR) $(RESOURCEDIR)
	$(CP) $(RESOURCE) $(RESOURCEDIR)
	$(COMPRESS)
	$(CP) $(SRCMAN)$(COMPRESSED_MANPAGE) $(MANDIR)
	$(MANDB)

# Rebuild the project
re: fclean all

# Uninstall the executable and resources
uninstall:
	$(RM) $(DESTDIR)$(NAME)
	$(RM) $(MANDIR)$(COMPRESSED_MANPAGE)
	$(RM) -r $(RESOURCEDIR)
	$(MANDB)

.PHONY: all clean fclean install re uninstall