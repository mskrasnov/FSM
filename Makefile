# (C) 2025 Michail Krasnov <mskrasnov07@ya.ru>

TARGET := x86_64-unknown-linux-gnu
BINARY_NAME := ferrix-app
POLKIT_BINARY := ferrix-polkit
RELEASE_DIR := ./target/$(TARGET)/release/
DESTDIR := /
INSTALL_DIR := $(DESTDIR)/usr/bin
POLICY_DIR := $(DESTDIR)/usr/share/polkit-1/actions
DESKTOP_DIR := $(DESTDIR)/usr/share/applications
ICON_DIR := $(DESTDIR)/usr/share/icons/hicolor/scalable/apps
SHARE_DIR := $(DESTDIR)/usr/share/Ferrix
DATA_DIR := ./ferrix-app/data

GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m

.PHONY: all build install uninstall clean help

all: build

build:
	@echo "$(YELLOW)Building Ferrix in release mode...$(NC)"
	cargo build --release --target=$(TARGET)
	@echo "$(GREEN)Build completed successfully!$(NC)"

deb:
	cargo deb --target=$(TARGET)

install: build
	@echo "$(YELLOW)Installing Ferrix...$(NC)"
	
	sudo install -Dm755 $(RELEASE_DIR)/$(POLKIT_BINARY) $(INSTALL_DIR)/$(POLKIT_BINARY)
	sudo install -Dm755 $(RELEASE_DIR)/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "$(GREEN)Binaries installed to $(INSTALL_DIR)$(NC)"
	
	sudo install -Dm644 $(DATA_DIR)/com.ferrix.policy $(POLICY_DIR)/com.ferrix.policy
	@echo "$(GREEN)Polkit policy installed$(NC)"
	
	sudo install -Dm644 $(DATA_DIR)/FSM.desktop $(DESKTOP_DIR)/FSM.desktop
	sudo install -Dm644 $(DATA_DIR)/com.mskrasnov.Ferrix.svg $(ICON_DIR)/com.mskrasnov.Ferrix.svg
	sudo install -Dm644 $(DATA_DIR)/com.mskrasnov.Ferrix.svg $(SHARE_DIR)/com.mskrasnov.Ferrix.svg
	@echo "$(GREEN)Desktop integration installed$(NC)"
	
	# Update icon cache (if gtk-update-icon-cache is available)
	@if command -v gtk-update-icon-cache >/dev/null 2>&1; then \
		echo "$(YELLOW)Updating icon cache...$(NC)"; \
		sudo gtk-update-icon-cache -q -t -f $(ICON_DIR)/../; \
		echo "$(GREEN)Icon cache updated$(NC)"; \
	else \
		echo "$(YELLOW)gtk-update-icon-cache not found, skipping icon cache update$(NC)"; \
	fi
	
	@echo "$(GREEN)Ferrix installed successfully!$(NC)"
	@echo "$(YELLOW)You can now run 'ferrix-app' from your application menu or terminal$(NC)"

uninstall:
	@echo "$(YELLOW)Uninstalling Ferrix...$(NC)"
	
	sudo rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	sudo rm -f $(INSTALL_DIR)/$(POLKIT_BINARY)
	@echo "$(GREEN)Binaries removed$(NC)"
	
	sudo rm -f $(POLICY_DIR)/com.ferrix.policy
	@echo "$(GREEN)Polkit policy removed$(NC)"
	
	sudo rm -f $(DESKTOP_DIR)/FSM.desktop
	sudo rm -f $(ICON_DIR)/com.mskrasnov.Ferrix.svg
	@echo "$(GREEN)Desktop integration removed$(NC)"
	
	@if command -v gtk-update-icon-cache >/dev/null 2>&1; then \
		echo "$(YELLOW)Updating icon cache...$(NC)"; \
		sudo gtk-update-icon-cache -q -t -f $(ICON_DIR)/../; \
		echo "$(GREEN)Icon cache updated$(NC)"; \
	fi
	
	@echo "$(GREEN)Ferrix uninstalled successfully!$(NC)"

clean:
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	cargo clean
	@echo "$(GREEN)Clean completed$(NC)"

run: build
	@echo "$(YELLOW)Running Ferrix...$(NC)"
	$(RELEASE_DIR)/$(BINARY_NAME)

run_debug:
	@echo "$(YELLOW)Running Ferrix in the $(GREEN)debug mode$(YELLOW)...$(NC)"
	cargo run --bin=ferrix-app --target=$(TARGET)

debug:
	@echo "$(YELLOW)Building in debug mode...$(NC)"
	cargo build --target=$(TARGET)
	@echo "$(GREEN)Debug build completed$(NC)"

help:
	@echo "Available targets:"
	@echo "  $(GREEN)build$(NC)     - Build the project in release mode (default)"
	@echo "  $(GREEN)install$(NC)   - Build and install system-wide"
	@echo "  $(GREEN)uninstall$(NC) - Remove installed files"
	@echo "  $(GREEN)clean$(NC)     - Remove build artifacts"
	@echo "  $(GREEN)run$(NC)       - Build and run without installing"
	@echo "  $(GREEN)debug$(NC)     - Build in debug mode"
	@echo "  $(GREEN)help$(NC)      - Show this help message"
	@echo ""
	@echo "Examples:"
	@echo "  make install           # Build and install"
	@echo "  make run               # Build and test locally"
	@echo "  make uninstall         # Remove from system"

.DEFAULT_GOAL := help
