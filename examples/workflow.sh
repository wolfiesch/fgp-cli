#!/bin/bash
# FGP CLI Workflow Examples
#
# This script demonstrates common FGP CLI usage patterns.
# Make sure daemons are running before executing these commands.

set -e

echo "FGP CLI Workflow Examples"
echo "========================="
echo

# Check which AI agents are installed
echo "1. Detecting installed AI agents..."
fgp agents
echo

# Show status of all daemons
echo "2. Checking daemon status..."
fgp status
echo

# Example: Gmail workflow
echo "3. Gmail workflow example (requires gmail daemon)..."
echo "   Start daemon: fgp start gmail"
echo "   Check inbox:  fgp call gmail.inbox -p '{\"limit\": 5}'"
echo "   Search:       fgp call gmail.search -p '{\"query\": \"is:unread\"}'"
echo

# Example: Browser automation
echo "4. Browser workflow example (requires browser daemon)..."
echo "   Start daemon: browser-gateway start"
echo "   Navigate:     browser-gateway open 'https://example.com'"
echo "   Snapshot:     browser-gateway snapshot"
echo "   Click:        browser-gateway click 'button'"
echo

# Example: GitHub workflow
echo "5. GitHub workflow example (requires github daemon)..."
echo "   Start daemon: fgp start github"
echo "   List repos:   fgp call github.repos -p '{\"limit\": 10}'"
echo "   Get issues:   fgp call github.issues -p '{\"repo\": \"owner/repo\"}'"
echo

# Example: Calendar workflow
echo "6. Calendar workflow example (requires calendar daemon)..."
echo "   Start daemon: fgp start calendar"
echo "   Today:        fgp call calendar.today"
echo "   Free slots:   fgp call calendar.free_slots -p '{\"duration_minutes\": 30}'"
echo

echo "For more information, see: https://github.com/fast-gateway-protocol"
